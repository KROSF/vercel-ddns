#!/usr/bin/env sh

set -euo pipefail

ENVS=`printenv | tr '\n' ' '`

echo "$VDDNS_DOMAIN $VDDNS_SUBDOMAIN $VERCEL_TOKEN" > /dev/null

echo "${CRON_SCHEDULE:=*/15 * * * *} $ENVS /usr/local/bin/vercel-ddns" > /var/spool/cron/crontabs/root

crond -f
