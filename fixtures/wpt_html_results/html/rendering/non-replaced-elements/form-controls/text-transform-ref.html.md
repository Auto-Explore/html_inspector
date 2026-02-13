# html/rendering/non-replaced-elements/form-controls/text-transform-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/text-transform-ref.html",
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
</head>
<body>
  <span>THIS TEXT SHOULD BE UPPER-CASE.</span><br />

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

  <select>
    <option>THIS TEXT SHOULD BE UPPER-CASE.</option>
  </select><br />
  <select multiple>
    <option>THIS TEXT SHOULD BE UPPER-CASE.</option>
  </select><br />
  <select multiple>
    <optgroup label="THIS TEXT SHOULD BE UPPER-CASE.">
      <option>THIS TEXT SHOULD BE UPPER-CASE.</option>
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/text-transform-ref.html"
}
```
