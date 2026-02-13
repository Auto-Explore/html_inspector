# html/semantics/forms/the-select-element/customizable-select/selected-option-parser-crash.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/selected-option-parser-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<select disabled>
  <option id=one selected>one</option>
  <option id=two disabled>two
    <table><div><div</table>
  </option>
  <option id=three selected>three</option>
</select>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “div<” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “div<” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.tokenizer.lt_expecting_attr_name",
      "message": "Saw “<” when expecting an attribute name. Probable cause: Missing “>” immediately before.",
      "severity": "Warning",
      "span": {
        "byte_end": 131,
        "byte_start": 119,
        "col": 17,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.select.selected.multiple_without_multiple",
      "message": "The “select” element cannot have more than one selected “option” descendant unless the “multiple” attribute is specified.",
      "severity": "Warning",
      "span": {
        "byte_end": 172,
        "byte_start": 146,
        "col": 3,
        "line": 7
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/selected-option-parser-crash.html"
}
```
