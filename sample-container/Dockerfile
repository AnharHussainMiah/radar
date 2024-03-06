FROM alpine
RUN apk add --no-cache --upgrade bash
COPY ./script.sh /root/script.sh
WORKDIR /root
RUN chmod +x /root/script.sh
ENTRYPOINT /root/script.sh