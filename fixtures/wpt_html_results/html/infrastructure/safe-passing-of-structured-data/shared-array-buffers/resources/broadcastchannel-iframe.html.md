# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/resources/broadcastchannel-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/resources/broadcastchannel-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>A test page that messes with a given SharedArrayBuffer sent from a BroadcastChannel</title>

<script>
"use strict";
const query = new URLSearchParams(location.search);
const channel = new BroadcastChannel(query.get("channel"));
const i = Number(query.get("index"));

channel.onmessage = e => {
  const sab = e.data.sab;
  if (sab === undefined) {
    // We only care about "broadcasts" from the window
    return;
  }

  const view = new Uint8Array(sab);
  view[i] = i + 1;
  channel.postMessage({ i });
};
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/resources/broadcastchannel-iframe.html"
}
```
