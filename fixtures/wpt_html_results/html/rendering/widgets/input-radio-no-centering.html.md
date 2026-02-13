# html/rendering/widgets/input-radio-no-centering.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-radio-no-centering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1441341">
<link rel="match" href="input-no-centering-ref.html">
<style>
input {
  appearance: none;
  margin: 0;
  width: 200px;
  height: 200px;
}
input:before {
  content: "";
  display: block;
  width: 100px;
  height: 100px;
  background: green;
}
</style>
<input type="radio">
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
  "source_name": "html/rendering/widgets/input-radio-no-centering.html"
}
```
