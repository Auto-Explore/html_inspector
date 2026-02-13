# html/rendering/widgets/input-checkbox-disabled-checked.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-checkbox-disabled-checked.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>checkbox with disabled and checked attributes renders differently than unchecked</title>
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1735077">
<link rel=mismatch href="input-checkbox-disabled-checked-notref.html">
<input type=checkbox disabled checked>
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
  "source_name": "html/rendering/widgets/input-checkbox-disabled-checked.html"
}
```
