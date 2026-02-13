# html/webappapis/system-state-and-capabilities/the-navigator-object/resources/handler.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/resources/handler.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<p>This popup can be closed if it does not close itself.
<p>
<script>
// This resource either gets navigated to through a service worker as a result of a URL that looks
// like:
// https://.../html/webappapis/system-state-and-capabilities/the-navigator-object/resources/handler/{type}/...
// (the host is excluded to not upset the lint tool)
// or it gets navigated to directly with the type appended to the end of the URL. In that case type
// can only be fragment or query.

let type = null;
let swString = null;
if (new URL(document.URL).pathname.endsWith("handler.html")) {
  swString = "";
  type = (document.URL.endsWith("fragment")) ? "fragment" : "query";
} else {
  type = document.URL.split("/")[9];
  swString = "sw";
}
new BroadcastChannel(`protocol-handler-${type}${swString}`).postMessage(document.URL);
window.close();
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/resources/handler.html"
}
```
