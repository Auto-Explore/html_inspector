# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-image-content-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-image-content-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Crash test: asynchronously applying image content to image input</title>
<link rel="author" href="mailto:yuzhehan@chromium.org" title="Yu Han">
<link rel="help" href="https://crbug.com/1096002">
<style>
  .content { content: url(data:text/plain,aaa); }
</style>
<input id="input" type="image" class=content>
<script>
  onload = ()=> {
    document.body.offsetTop;
    input.setAttribute('class', '');
    document.body.offsetTop;
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 330,
        "byte_start": 285,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/input-image-content-crash.html"
}
```
