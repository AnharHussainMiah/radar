use quickjs_rs::Context;

/* -------------------------------------------------------------------------------------------------
| this is horrible but works, because the JSON returned from the docker unix socket is deeply nested
| doing all these transformation is too much work right now and I'm super lazy
| instead I've shamelessy grabbed code from:
| https://github.com/nexdrew/rekcod
| and just ripped out the transformation function, it's in JavaScript so I'm usng quickJS to
| basically run this JavaScript transformation function
| yeah it's dirty but it works.
| One day, it can be ripped out and done natively, but today is not that day
------------------------------------------------------------------------------------------------- */
pub fn transform(data: String) -> Result<String, String> {
    let context = Context::new().map_err(|e| e.to_string())?;
    let code = r#"
    function toRunCommand (inspectObj) {
        let rc = append('docker run', '--name', inspectObj.Name.replace("/", ""))
      
        const hostcfg = inspectObj.HostConfig || {}
        const networkMode = hostcfg.NetworkMode
        const utsMode = hostcfg.UTSMode
        const modes = { networkMode, utsMode }
      
        rc = appendBoolean(rc, hostcfg.Privileged, '--privileged') // fixes #49
        // TODO something about devices or capabilities instead of privileged?
        // --cap-add: Add Linux capabilities
        // --cap-drop: Drop Linux capabilities
        // --device=[]: Allows you to run devices inside the container without the --privileged flag
        // see https://docs.docker.com/engine/reference/run/#runtime-privilege-and-linux-capabilities
      
        if (hostcfg.Runtime) rc = append(rc, '--runtime', hostcfg.Runtime)
        rc = appendArray(rc, '-v', hostcfg.Binds)
        rc = appendArray(rc, '--volumes-from', hostcfg.VolumesFrom)
        if (hostcfg.PortBindings && isCompatible('-p', modes)) {
          rc = appendObjectKeys(rc, '-p', hostcfg.PortBindings, ipPort => {
            return ipPort.HostIp ? ipPort.HostIp + ':' + ipPort.HostPort : ipPort.HostPort
          })
        }
        rc = appendArray(rc, '--link', hostcfg.Links, link => {
          link = link.split(':')
          if (link[0] && ~link[0].lastIndexOf('/')) link[0] = link[0].substring(link[0].lastIndexOf('/') + 1)
          if (link[1] && ~link[1].lastIndexOf('/')) link[1] = link[1].substring(link[1].lastIndexOf('/') + 1)
          return link[0] + ':' + link[1]
        })
        if (hostcfg.PublishAllPorts && isCompatible('-P', modes)) rc = rc + ' -P'
      
        if (networkMode && networkMode !== 'default') {
          rc = append(rc, '--net', networkMode)
        }
        if (utsMode && isCompatible('--uts', modes)) {
          rc = append(rc, '--uts', utsMode)
        }
        if (hostcfg.RestartPolicy && hostcfg.RestartPolicy.Name) {
          rc = append(rc, '--restart', hostcfg.RestartPolicy, policy => {
            return policy.Name === 'on-failure' ? policy.Name + ':' + policy.MaximumRetryCount : policy.Name
          })
        }
        if (isCompatible('--add-host', modes)) rc = appendArray(rc, '--add-host', hostcfg.ExtraHosts)
        rc = appendArray(rc, '--group-add', hostcfg.GroupAdd)
        if (hostcfg.PidMode) rc = append(rc, '--pid', hostcfg.PidMode)
        rc = appendArray(rc, '--security-opt', hostcfg.SecurityOpt, quote)
      
        const cfg = inspectObj.Config || {}
      
        if (cfg.Hostname && isCompatible('-h', modes)) rc = append(rc, '-h', cfg.Hostname)
        if (cfg.Domainname && isCompatible('--domainname', modes)) rc = append(rc, '--domainname', cfg.Domainname)
      
        if (cfg.ExposedPorts && isCompatible('--expose', modes)) {
          rc = appendObjectKeys(rc, '--expose', cfg.ExposedPorts)
        }
        if (cfg.Labels) {
          // rc = appendObjectEntries(rc, '-l', cfg.Labels, '=')
          rc = appendObjectKeys(rc, '-l', cfg.Labels)
        }
        rc = appendArray(rc, '-e', cfg.Env, quote)
        rc = appendConfigBooleans(rc, cfg)
        if (cfg.Entrypoint) rc = appendJoinedArray(rc, '--entrypoint', cfg.Entrypoint, ' ')
      
        rc = rc + ' ' + (cfg.Image || inspectObj.Image)
      
        if (cfg.Cmd) rc = appendJoinedArray(rc, null, cfg.Cmd, ' ')
      
        return rc
      }
      
      // The following options are invalid in 'container' NetworkMode:
      // --add-host
      // -h, --hostname
      // --dns
      // --dns-search
      // --dns-option
      // --mac-address
      // -p, --publish
      // -P, --publish-all
      // --expose
      // The following options are invalid in 'host' UTSMode:
      // -h, --hostname
      // --domainname
      function isCompatible (flag, modes) {
        switch (flag) {
          case '-h':
            return !(modes.networkMode || '').startsWith('container:') && modes.utsMode !== 'host'
          case '--add-host':
          case '--dns':
          case '--dns-search':
          case '--dns-option':
          case '--mac-address':
          case '-p':
          case '-P':
          case '--expose':
            return !(modes.networkMode || '').startsWith('container:')
          case '--domainname':
            return modes.utsMode !== 'host'
          default:
            return true
        }
      }
      
      function quote (str) {
        return '\'' + str.replace(/'/g, '\'\\\'\'') + '\''
      }
      
      function appendConfigBooleans (str, cfg) {
        const stdin = cfg.AttachStdin === true
        const stdout = cfg.AttachStdout === true
        const stderr = cfg.AttachStderr === true
        str = appendBoolean(str, !stdin && !stdout && !stderr, '-d')
        str = appendBoolean(str, stdin, '-a', 'stdin')
        str = appendBoolean(str, stdout, '-a', 'stdout')
        str = appendBoolean(str, stderr, '-a', 'stderr')
        str = appendBoolean(str, cfg.Tty === true, '-t')
        str = appendBoolean(str, cfg.OpenStdin === true, '-i')
        return str
      }
      
      function appendBoolean (str, bool, key, val) {
        return bool ? (val ? append(str, key, val) : str + ' ' + key) : str
      }
      
      function appendJoinedArray (str, key, array, join) {
        if (!Array.isArray(array)) return str
      
        // --entrypoint "tini -- /docker-entrypoint.sh"
        if (key) return append(str, key, array.join(join), joined => '"' + joined + '"')
      
        // 'sh' '-c' '(a -a) && (b -b)'
        return append(str, key, array.map(quote).join(join))
      }
      
      function appendObjectKeys (str, key, obj, transformer) {
        let newStr = str
        Object.keys(obj).forEach(k => {
          newStr = append(newStr, key, { key: k, val: obj[k] }, agg => {
            if (!agg.val) return agg.key
            let v = ''
            if (Array.isArray(agg.val)) {
              // used for PortBindings
              agg.val.forEach(valObj => {
                v = (typeof transformer === 'function' ? transformer(valObj) : valObj)
              })
            } else if (typeof agg.val === 'string') {
              // used for Labels
              return agg.key + '=' + quote(agg.val)
            }
            // prefix used for PortBindings, key-only used for ExposedPorts
            return (v ? v + ':' : '') + agg.key
          })
        })
        return newStr
      }
      
      function appendArray (str, key, array, transformer) {
        if (!Array.isArray(array)) return str
        let newStr = str
        array.forEach(v => {
          newStr = append(newStr, key, v, transformer)
        })
        return newStr
      }
      
      function append (str, key, val, transformer) {
        if (!val) return str
        return str + ' ' + (key ? key + ' ' : '') + (typeof transformer === 'function' ? transformer(val) : val)
      }

      toRunCommand(<<INJECT>>)
    "#;
    let injected = code.replace("<<INJECT>>", &data);
    let value = context
        .eval_as::<String>(&injected)
        .map_err(|e| e.to_string())?;
    Ok(value)
}
