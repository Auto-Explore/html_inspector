# html/semantics/interestfor/interestelement-interface.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestelement-interface.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<link rel="author" title="Keith Cirkel" href="mailto:keithamus@github.com" >
<link rel="author" title="Luke Warlow" href="mailto:lwarlow@igalia.com" >
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<button id="buttonInvoker" interestfor="interestee"></button>
<a id="aInvoker" interestfor="interestee"></a>
<div id="interestee"></div>

<script>
  test(function () {
    assert_equals(buttonInvoker.interestForElement, interestee);
    assert_equals(aInvoker.interestForElement, interestee);
  }, "interestForElement reflects interestee HTML element");

  test(function () {
    const div = document.body.appendChild(document.createElement("div"));
    buttonInvoker.interestForElement = div;
    aInvoker.interestForElement = div;
    assert_equals(buttonInvoker.interestForElement, div);
    assert_equals(buttonInvoker.getAttribute("interestfor"), "");
    assert_equals(aInvoker.interestForElement, div);
    assert_equals(aInvoker.getAttribute("interestfor"), "");
  }, "interestForElement reflects set value");

  test(function () {
    const host = document.body.appendChild(document.createElement("div"));
    const shadow = host.attachShadow({ mode: "open" });
    const button = shadow.appendChild(document.createElement("button"));
    button.interestForElement = interestee;
    assert_equals(button.interestForElement, interestee);
    assert_equals(buttonInvoker.getAttribute("interestfor"), "");
  }, "interestForElement reflects set value across shadow root into light dom");

  test(function () {
    const host = document.body.appendChild(document.createElement("div"));
    const shadow = host.attachShadow({ mode: "open" });
    const div = shadow.appendChild(document.createElement("div"));
    buttonInvoker.interestForElement = div;
    assert_equals(buttonInvoker.interestForElement, null);
    assert_equals(buttonInvoker.getAttribute("interestfor"), "");
  }, "interestForElement does not reflect set value inside shadowroot");

  test(function () {
    buttonInvoker.setAttribute('interestfor', 'invalid');
    assert_equals(buttonInvoker.interestForElement, null);
    assert_equals(buttonInvoker.getAttribute("interestfor"), "invalid");
    aInvoker.setAttribute('interestfor', 'invalid');
    assert_equals(aInvoker.interestForElement, null);
    assert_equals(aInvoker.getAttribute("interestfor"), "invalid");
  }, "interestForElement does not reflect invalid value");

  test(function () {
    assert_throws_js(
      TypeError,
      function () {
        buttonInvoker.interestForElement = {};
      },
      "interestForElement attribute value must be an instance of Element",
    );
    assert_throws_js(
      TypeError,
      function () {
        aInvoker.interestForElement = {};
      },
      "interestForElement attribute value must be an instance of Element",
    );
  }, "interestForElement throws error on assignment of non Element");
</script>
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
  "source_name": "html/semantics/interestfor/interestelement-interface.tentative.html"
}
```
