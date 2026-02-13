# html/semantics/forms/the-select-element/customizable-select/nested-select-crash.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/nested-select-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/373893049">

<style>
select,::picker(select) {
  appearance: base-select;
}
</style>

<select>
  <button>
    Hello??
    <selectedcontent />
  </button>
  <option>
    <object type="no/type">
      <select>
        <button>
          Hello??
          <selectedcontent />
        </button>
        <option>One</option>
        <option>Two</option>
      </select>
    </object>
  </option>
  <option>Two</option>
</select>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parse.self_closing.non_void",
      "message": "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
      "severity": "Error",
      "span": {
        "byte_end": 265,
        "byte_start": 246,
        "col": 5,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 316,
        "byte_start": 293,
        "col": 5,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parse.self_closing.non_void",
      "message": "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
      "severity": "Error",
      "span": {
        "byte_end": 396,
        "byte_start": 377,
        "col": 11,
        "line": 21
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/nested-select-crash.html"
}
```
