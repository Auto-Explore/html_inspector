# html/rendering/replaced-elements/the-option-element/option-label-whitespace.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-option-element/option-label-whitespace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/10955">
<link rel=match href="option-label-whitespace-ref.html">

<select multiple>
  <option>no label attribute</option>
  <option label="">empty label attribute</option>
  <option label="  ">whitespace label attribute</option>
</select>
<br>

<select>
  <option label="">empty label attribute</option>
</select>
<br>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.label.empty",
      "message": "Bad value “” for attribute “label” on element “option”.",
      "severity": "Warning",
      "span": {
        "byte_end": 268,
        "byte_start": 251,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.option.label.empty",
      "message": "Bad value “” for attribute “label” on element “option”.",
      "severity": "Warning",
      "span": {
        "byte_end": 400,
        "byte_start": 383,
        "col": 3,
        "line": 14
      }
    },
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
  "source_name": "html/rendering/replaced-elements/the-option-element/option-label-whitespace.html"
}
```
