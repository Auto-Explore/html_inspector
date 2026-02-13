# html/cross-origin-opener-policy/resources/resource-cleanup.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/resources/resource-cleanup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<meta charset="utf-8">
<title>Redirect destination for non-HTML documents to close themselves</title>
<script>
'use strict';
const channel = new URL(location).searchParams.get('channel');
const bc = new BroadcastChannel(channel);
bc.onmessage = () => close();
bc.postMessage({name: 'FAIL', closed: 'FAIL' });
</script>
```

```json
{
  "messages": [],
  "source_name": "html/cross-origin-opener-policy/resources/resource-cleanup.html"
}
```
