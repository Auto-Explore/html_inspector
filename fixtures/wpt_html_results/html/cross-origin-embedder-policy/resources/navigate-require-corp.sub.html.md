# html/cross-origin-embedder-policy/resources/navigate-require-corp.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/resources/navigate-require-corp.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
  let current = new URL(window.location.href);
  let navigateTo = current.searchParams.get("to");
  let channelName = current.searchParams.get("channelName");
  let clearOpener = current.searchParams.get("clearOpener");

  if (clearOpener) {
    window.opener = null;
  }

  current.search = "";
  if (navigateTo) {
    let next = new URL(navigateTo, current);
    window.addEventListener("load", () => {
      window.location.href = next.href;
    });
  }

  if (channelName) {
    let bc = new BroadcastChannel(channelName);
    bc.postMessage("loaded");
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/cross-origin-embedder-policy/resources/navigate-require-corp.sub.html"
}
```
