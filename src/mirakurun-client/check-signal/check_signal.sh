#!/usr/bin/env bash

# Skip if env values are not present.
if [ -z $CHECK_SIGNAL_CHANNELS ] || [ -z $DISCORD_WEBHOOK_URL ]; then
    echo "Environment values are not present."
    exit
fi

set -eu
cd /

# Buffer to tmp file because of tsselect's broken stdin functionary.
TEST_M2TS_PATH="/tmp/test.m2ts"

# Loop CHECK_SIGNAL_CHANNELS with comma.
CHANNELS=(${CHECK_SIGNAL_CHANNELS//,/ })
for channel in "${CHANNELS[@]}"; do
    echo "Checking signal in $channel..."

    # Resolve target service name
    SERVICE_NAME=`mirakurun-client localhost /channels/$channel/services | jq -r ".[0].name"`
    if [ $? -ne 0 ]; then
        continue
    fi
    echo "Resolved $channel for $SERVICE_NAME."

    # Check signal of target channel.
    echo "Recording $channel for 10 seconds..."
    set +eu
    timeout 10 mirakurun-client localhost /channels/$channel/stream > $TEST_M2TS_PATH
    set -eu

    time=`date "+%Y/%m/%d %H:%M:%S"`

    # Skip if failed to record.
    if [[ ! -s $TEST_M2TS_PATH ]]; then
        continue
    fi

    tsselect=`tsselect $TEST_M2TS_PATH`
    rm -f $TEST_M2TS_PATH

    # If there are drop/error/scrambling data.
    line=`echo $tsselect | grep -v "d=\s*0, e=\s*0, scrambling=\s*0" | wc -l`
    if [ $line -ne 0 ]; then
        echo "tsselect returned some drop/error/scrambling count."
        curl -s \
            -H "Content-Type: application/json" \
            -d "{\"username\": \"Check Signal\", \"content\": \"[$time] \`$SERVICE_NAME ($channel)\` のストリームに問題が見つかりました。\n\`\`\`c\n$tsselect\n\`\`\`\"}" \
            -X POST \
            $DISCORD_WEBHOOK_URL
    fi
done
