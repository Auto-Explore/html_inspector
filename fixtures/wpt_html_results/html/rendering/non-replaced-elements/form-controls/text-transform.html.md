# html/rendering/non-replaced-elements/form-controls/text-transform.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/text-transform.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <link rel="match" href="text-transform-ref.html">
</head>
<body style="text-transform: uppercase;">
  <span>this text should be upper-case.</span><br />

  <input type="text" value="this text should be lower-case."><br />

  <select>
    <option>this text should be lower-case.</option>
  </select><br />
  <select multiple>
    <option>this text should be lower-case.</option>
  </select><br />
  <select multiple>
    <optgroup label="this text should be lower-case.">
      <option>this text should be lower-case.</option>
    </optgroup>
  </select><br />

  <select style="text-transform: uppercase;">
    <option>this text should be upper-case.</option>
  </select><br />
  <select multiple style="text-transform: uppercase;">
    <option>this text should be upper-case.</option>
  </select><br />
  <select multiple style="text-transform: uppercase;">
    <optgroup label="this text should be upper-case.">
      <option>this text should be upper-case.</option>
    </optgroup>
  </select><br />

  <button>this text should be lower-case.</button><br />

  <textarea>this text should be lower-case.</textarea><br />
</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/text-transform.html"
}
```
