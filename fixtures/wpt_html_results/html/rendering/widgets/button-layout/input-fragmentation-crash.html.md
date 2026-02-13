# html/rendering/widgets/button-layout/input-fragmentation-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/input-fragmentation-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1978487">
<style>
.parent {
  columns: 2;
}
input {
  float: inline-start;
}
</style>
<div class="parent">
  <input style="display: block" type="color">
</div>
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
  "source_name": "html/rendering/widgets/button-layout/input-fragmentation-crash.html"
}
```
