# html/semantics/forms/the-select-element/select-in-table-crash.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/select-in-table-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1519397">
<table><select>A0A0AA<<datalist><title></title><table><table><td>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.select_in_table",
      "message": "Start tag “select” seen in “table”.",
      "severity": "Error",
      "span": {
        "byte_end": 168,
        "byte_start": 160,
        "col": 8,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.bad_char_after_lt",
      "message": "Bad character “<” after “<”. Probable cause: Unescaped “<”. Try escaping it as “&lt;”.",
      "severity": "Warning",
      "span": {
        "byte_end": 176,
        "byte_start": 174,
        "col": 22,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 192,
        "byte_start": 185,
        "col": 33,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nested_table",
      "message": "Start tag for “table” seen but the previous “table” is still open.",
      "severity": "Error",
      "span": {
        "byte_end": 207,
        "byte_start": 200,
        "col": 48,
        "line": 4
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
  "source_name": "html/semantics/forms/the-select-element/select-in-table-crash.html"
}
```
