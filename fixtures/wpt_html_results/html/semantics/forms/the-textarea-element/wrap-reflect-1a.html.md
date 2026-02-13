# html/semantics/forms/the-textarea-element/wrap-reflect-1a.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/wrap-reflect-1a.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Dynamic manipulation of textarea.wrap</title>
<link rel=match href=wrap-reflect-1-ref.html>
<link rel=help href=https://html.spec.whatwg.org/multipage/#dom-textarea-wrap>
<link rel=author title=Ms2ger href=mailto:ms2ger@gmail.com>
<textarea wrap=off cols=20>01234567890 01234567890 01234567890</textarea>
<script>
document.getElementsByTagName("textarea")[0].wrap = "soft";
</script>
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
  "source_name": "html/semantics/forms/the-textarea-element/wrap-reflect-1a.html"
}
```
