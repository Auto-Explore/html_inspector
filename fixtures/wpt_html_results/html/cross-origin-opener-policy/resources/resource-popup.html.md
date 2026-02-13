# html/cross-origin-opener-policy/resources/resource-popup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/resources/resource-popup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<meta charset="utf-8">
<title></title>
<script>
'use strict';
const params = new URL(location).searchParams;
const bc = new BroadcastChannel(params.get('channel_name'));
const win = open(params.get('resource'), params.get('resource_name'));

bc.onmessage = () => {
  win.close();
  close();
};
const id = setInterval(() => {
  if (win.closed || win.location.href !== 'about:blank') {
    clearInterval(id);
    const winName = (() => {
      try {
        return win.name;
      } catch (e) {
        return null;
      }
    })();
    bc.postMessage({name: winName || null, closed: win.closed});
  }
}, 100);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 63,
        "byte_start": 56,
        "col": 1,
        "line": 4
      }
    }
  ],
  "source_name": "html/cross-origin-opener-policy/resources/resource-popup.html"
}
```
