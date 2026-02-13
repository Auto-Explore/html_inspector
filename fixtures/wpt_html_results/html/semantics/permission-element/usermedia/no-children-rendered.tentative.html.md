# html/semantics/permission-element/usermedia/no-children-rendered.tentative.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/usermedia/no-children-rendered.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<link rel="help" href="https://github.com/WICG/PEPC/blob/main/explainer.md#locking-the-pepc-style">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<!-- The usermedia element is not a void element. Its children do not render. -->

<!-- This tests for implementations that used to have no end tag for the usermedia element
 In those implementations there would be 2 usermedia elements in this div. -->
<div id="usermedia-element-div">
  <usermedia type="camera">
  <usermedia type="microphone">
</div>

<usermedia id="mic" type="microphone">This is some text</usermedia>

<!-- The usermedia element does not automatically close <p> tags -->
<p id="paragraph">Foo <usermedia type="camera">bar</usermedia> baz</p>

<script>
  test(function(){
    assert_equals(1, document.getElementById("usermedia-element-div").childElementCount);
    assert_equals('', document.getElementById("mic").innerText);
    assert_equals('This is some text', document.getElementById("mic").textContent);
    assert_equals('This is some text', document.getElementById("mic").innerHTML);
    assert_equals(4, document.body.childElementCount); //div, usermedia, script, p
    assert_equals('Foo bar baz', document.getElementById("paragraph").textContent);
  }, "The usermedia element should have no end tag or contents");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “usermedia” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 563,
        "byte_start": 538,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “usermedia” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 563,
        "byte_start": 538,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “usermedia” not allowed as child of “usermedia” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 595,
        "byte_start": 566,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “usermedia” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 595,
        "byte_start": 566,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “usermedia” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 642,
        "byte_start": 604,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “usermedia” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 642,
        "byte_start": 604,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “usermedia” not allowed as child of “p” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 764,
        "col": 23,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “usermedia” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 789,
        "byte_start": 764,
        "col": 23,
        "line": 19
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
  "source_name": "html/semantics/permission-element/usermedia/no-children-rendered.tentative.html"
}
```
