# html/editing/the-hidden-attribute/hidden-until-found-005-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/hidden-until-found-005-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
<meta charset="utf8">
<title>hidden=until-found and size contained (reference)</title>
<link rel="author" title="Vladimir Levin" href="mailto:vmpstr@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#attr-hidden-until-found">

<style>
div {
  border: 1px solid black;
}
</style>

Test passes if there is a empty div with border below.
<div></div>
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
  "source_name": "html/editing/the-hidden-attribute/hidden-until-found-005-ref.html"
}
```
