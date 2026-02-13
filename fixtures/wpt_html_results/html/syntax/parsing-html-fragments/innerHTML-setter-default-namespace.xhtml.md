# html/syntax/parsing-html-fragments/innerHTML-setter-default-namespace.xhtml

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing-html-fragments/innerHTML-setter-default-namespace.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="utf-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<span xmlns="someNamespace" xmlns:html="http://www.w3.org/1999/xhtml">
    <html:span id="target"/>
</span>
<script>
<![CDATA[

test(() => {
    const element = document.getElementById("target");
    element.innerHTML = '<b /><html:b />';
    assert_equals(element.firstChild.prefix, null);
    assert_equals(element.firstChild.namespaceURI, "someNamespace");
    assert_equals(element.lastChild.prefix, 'html');
    assert_equals(element.lastChild.namespaceURI, "http://www.w3.org/1999/xhtml");
}, "Setting innerHTML on a HTML element with a non-HTML namespace as the default namespace");

test(() => {
    const element = document.getElementById("target");
    element.outerHTML = '<b /><html:b />';
    assert_equals(element.firstChild.prefix, null);
    assert_equals(element.firstChild.namespaceURI, "someNamespace");
    assert_equals(element.lastChild.prefix, 'html');
    assert_equals(element.lastChild.namespaceURI, "http://www.w3.org/1999/xhtml");
}, "Setting outerHTML on a HTML element with a non-HTML namespace as the default namespace");

]]>
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “html:span” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 286,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “html:span” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 286,
        "col": 5,
        "line": 9
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
  "source_name": "html/syntax/parsing-html-fragments/innerHTML-setter-default-namespace.xhtml"
}
```
