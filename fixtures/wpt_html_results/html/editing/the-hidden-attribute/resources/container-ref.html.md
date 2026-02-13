# html/editing/the-hidden-attribute/resources/container-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/resources/container-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
<meta charset="utf8">
<title>Container (reference)</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">

<style>
#container {
  width: 150px;
  height: 150px;
  background: lightblue;
}
</style>

<div id=container></div>

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
  "source_name": "html/editing/the-hidden-attribute/resources/container-ref.html"
}
```
