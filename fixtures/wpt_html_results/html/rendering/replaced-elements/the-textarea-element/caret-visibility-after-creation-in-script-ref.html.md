# html/rendering/replaced-elements/the-textarea-element/caret-visibility-after-creation-in-script-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-textarea-element/caret-visibility-after-creation-in-script-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>A cursor should be shown when a textarea element is created and focused immediately</title>
<textarea id="target"></textarea>
<script>
  target.focus();
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
  "source_name": "html/rendering/replaced-elements/the-textarea-element/caret-visibility-after-creation-in-script-ref.html"
}
```
