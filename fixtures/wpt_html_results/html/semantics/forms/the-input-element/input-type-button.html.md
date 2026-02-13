# html/semantics/forms/the-input-element/input-type-button.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-type-button.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<head>
<title>input type button</title>
<link rel="author" title="Takeharu.Oshida" href="mailto:georgeosddev@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#button-state-(type=button)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<div id="hide" style="display">
  <input type="button"/>
  <input type="button" value="BUTTON"/>
  <form action="/" method="get" onsubmit="isSubmitted = true;return false;">
    <input type="button" value="mutable"/>
  </form>
  <form action="/" method="get" onsubmit="isSubmitted = true;return false;">
    <input type="button" value="immutable" disabled/>
  </form>
</div>
<script>
var isSubmitted = false;
var buttons = document.getElementsByTagName("input");

test(function() {
  assert_equals(buttons[0].click(), undefined, "The input element represents a button with no default behavior");
},"default behavior");

test(function() {
  assert_equals(buttons[0].value, "", "It must be the empty string");
},"empty value attribute");

test(function() {
  document.getElementById("hide").style.display = "block";
  assert_not_equals(buttons[0].offsetWidth, buttons[1].offsetWidth, "If the element has a value attribute, the button's label must be the value of that attribute");
  document.getElementById("hide").style.display = "none";
},"label value");

test(function() {
  isSubmitted = false;
  buttons[2].click();
  assert_equals(isSubmitted, false, "If the element is mutable, the element's activation behavior is to do nothing.");
},"mutable element's activation behavior is to do nothing.");

test(function() {
  isSubmitted = false;
  buttons[3].click()
  assert_equals(isSubmitted, false, "If the element is immutable, the element has no activation behavior.");
},"immutable element has no activation behavior.");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.button.value.nonempty",
      "message": "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
      "severity": "Warning",
      "span": {
        "byte_end": 427,
        "byte_start": 405,
        "col": 3,
        "line": 12
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
  "source_name": "html/semantics/forms/the-input-element/input-type-button.html"
}
```
