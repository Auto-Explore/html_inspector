# html/editing/the-hidden-attribute/resources/spacer-and-container-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/resources/spacer-and-container-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
<meta charset="utf8">
<title>Spacer and a container (reference)</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">

<style>
.spacer {
  width: 150px;
  height: 3000px;
  background: lightblue;
}
#container {
  contain: style layout;
  width: 150px;
  height: 150px;
}
</style>

<div class="spacer"></div>
<div id="container"></div>
</html>
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
  "source_name": "html/editing/the-hidden-attribute/resources/spacer-and-container-ref.html"
}
```
