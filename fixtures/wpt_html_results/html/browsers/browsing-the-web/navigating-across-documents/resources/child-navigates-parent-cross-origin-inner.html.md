# html/browsers/browsing-the-web/navigating-across-documents/resources/child-navigates-parent-cross-origin-inner.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/resources/child-navigates-parent-cross-origin-inner.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
const params = new URL(window.location).searchParams;
const property = params.get("property");

try {
  if (property === null) {
    parent.location = "foo";
  } else if (property === "reload") {
    parent.location.reload();
  } else if (property === "replace") {
    parent.location.replace("foo");
  } else {
    parent.location[property] = "foo";
  }
  parent.parent.postMessage("success", "*");
} catch (e) {
  parent.parent.postMessage(`error: ${e.name}`, "*");
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/resources/child-navigates-parent-cross-origin-inner.html"
}
```
