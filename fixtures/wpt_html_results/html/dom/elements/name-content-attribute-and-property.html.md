# html/dom/elements/name-content-attribute-and-property.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/name-content-attribute-and-property.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Only certain HTML elements reflect the name content attribute as a property</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
<div id="log"></div>
<script>
  function doesReflect(tagName) {
    var element = document.createElement(tagName);
    element.setAttribute("name", "foo");
    assert_equals(element.getAttribute("name"), "foo", "setAttribute should change content attribute");
    element.name = "bar";
    assert_equals(element.getAttribute("name"), "bar", "assignment to .name should change content attribute");
  }

  function doesNotReflect(tagName) {
    var element = document.createElement(tagName);
    element.setAttribute("name", "foo");
    assert_equals(element.getAttribute("name"), "foo", "setAttribute should change content attribute");
    element.name = "bar";
    assert_equals(element.getAttribute("name"), "foo", "assignment to .name should not change content attribute");
  }

  var reflectingTagNames = [
    "a", "button", "embed", "fieldset", "form", "frame",
    "iframe", "img", "input", "map", "meta", "object", "output",
    "param", "select", "slot", "textarea",
  ];

  // Optionally add "details" to reflectingTagNames since Chromium is
  // prototyping the proposal at
  // https://open-ui.org/components/accordion.explainer that adds this
  // reflection to "details" as well.
  // TODO(https://crbug.com/1444057): This runtime check should eventually be
  // removed, and depending on the outcome of the proposal details should
  // possibly be added to reflectingTagNames unconditionally.
  if ("name" in HTMLDetailsElement.prototype) {
    reflectingTagNames.push("details");
  }

  const old_and_new_elements = [...HTML5_ELEMENTS, ...HTML5_DEPRECATED_ELEMENTS];

  const nonReflectingTagNames = old_and_new_elements.filter(x => !reflectingTagNames.includes(x));

  reflectingTagNames.forEach(function(tagName) {
    test(function() {
      doesReflect(tagName)
    }, tagName + " element's name property reflects its content attribute");
  });

  nonReflectingTagNames.forEach(function(tagName) {
    test(function() {
      doesNotReflect(tagName)
    }, tagName + " element's name property does not reflect its content attribute");
  });
</script>

```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/elements/name-content-attribute-and-property.html"
}
```
