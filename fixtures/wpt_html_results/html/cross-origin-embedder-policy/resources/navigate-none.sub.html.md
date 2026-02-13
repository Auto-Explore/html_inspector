# html/cross-origin-embedder-policy/resources/navigate-none.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/resources/navigate-none.sub.html",
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
  let postMessageTo = current.searchParams.get("postMessageTo");
  current.search = "";
  if (navigateTo) {
    let next = new URL(navigateTo, current);
    window.addEventListener("load", () => {
      window.location.href = next.href;
    });
  }

  let target = undefined;
  if (channelName) {
    target = new BroadcastChannel(channelName);
  } else if (postMessageTo) {
    target = eval(postMessageTo);
  }

  if (target) {
    // Broadcast only once the DOM is loaded, so that the caller can
    // access reliably this document's body.
    window.addEventListener("DOMContentLoaded", () =>
      target.postMessage("loaded", "*"));

    // The page can also be restored from the back-forward cache:
    window.addEventListener('pageshow', function(event) {
      if (event.persisted)
        target.postMessage("loaded", "*");
    });
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
  "source_name": "html/cross-origin-embedder-policy/resources/navigate-none.sub.html"
}
```
