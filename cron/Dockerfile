FROM krosf/vercel-ddns

ADD entrypoint.sh /app/
RUN chmod +x /app/entrypoint.sh
WORKDIR /app/

ENTRYPOINT ["sh", "/app/entrypoint.sh"]
CMD [""]
