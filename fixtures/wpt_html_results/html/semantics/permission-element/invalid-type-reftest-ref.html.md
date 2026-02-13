# html/semantics/permission-element/invalid-type-reftest-ref.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/invalid-type-reftest-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<body>
  <div>
    The permission element behaves the same as an unknown element.
  </div>

<style>
  #id1 {
    background-color: hsla(210, 70%, 60%, 0.3); /* Semi-transparent blue */
    border-radius: 5px;
    padding: 2px 5px;
    border: solid 1px #767676;

    box-sizing: content-box;
    display: inline-block;
  }
  #id2 {
    background-color: azure;
    border: 1px dashed #ccc;
    padding: 3px;
    border-radius: 3px;

    box-sizing: content-box;
    display: inline-block;
  }
  button {
    font-family: "Comic Sans MS", cursive, sans-serif; /* Just for fun! */
    font-style: italic;
    font-weight: bold;
    color: #800080; /* Purple */
    text-decoration: underline dotted hotpink;
  }
</style>

<div>
  <foobar id="id1">This is some contents</foobar>
</div>
<div>
  <foobar id="id2">
    <button>A button</button>
    <span>
      <li>
        <ul>1</ul>
        <ul>2</ul>
      </li>
      Some text
    </span>
  </foobar>
</div>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 136,
        "byte_start": 129,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “foobar” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 782,
        "byte_start": 765,
        "col": 3,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “foobar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 782,
        "byte_start": 765,
        "col": 3,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “foobar” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 845,
        "byte_start": 828,
        "col": 3,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “foobar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 845,
        "byte_start": 828,
        "col": 3,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 897,
        "byte_start": 893,
        "col": 7,
        "line": 43
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
  "source_name": "html/semantics/permission-element/invalid-type-reftest-ref.html"
}
```
