# html/semantics/the-button-element/command-and-commandfor/interface.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/interface.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<button id="invoker" commandfor="invokee" command="test"></button>
<div id="invokee"></div>

<script>
  test(function () {
    assert_equals(invoker.commandForElement, invokee);
  }, "commandForElement reflects invokee HTML element");

  test(function () {
    const div = document.body.appendChild(document.createElement("div"));
    invoker.commandForElement = div;
    assert_equals(invoker.commandForElement, div);
    assert_equals(invoker.getAttribute("commandfor"), "");
    assert_equals(invoker.getAttribute("command"), "test");
  }, "commandForElement reflects set value");

  test(function () {
    const host = document.body.appendChild(document.createElement("div"));
    const shadow = host.attachShadow({ mode: "open" });
    const button = shadow.appendChild(document.createElement("button"));
    button.commandForElement = invokee;
    assert_equals(button.commandForElement, invokee);
    assert_equals(invoker.getAttribute("commandfor"), "");
    assert_equals(invoker.getAttribute("command"), "test");
  }, "commandForElement reflects set value across shadow root into light dom");

  test(function () {
    const host = document.body.appendChild(document.createElement("div"));
    const shadow = host.attachShadow({ mode: "open" });
    const div = shadow.appendChild(document.createElement("div"));
    invoker.commandForElement = div;
    assert_equals(invoker.commandForElement, null);
    assert_equals(invoker.getAttribute("commandfor"), "");
    assert_equals(invoker.getAttribute("command"), "test");
  }, "commandForElement does not reflect set value inside shadowroot");

  test(function () {
    assert_throws_js(
      TypeError,
      function () {
        invoker.commandForElement = {};
      },
      "commandForElement attribute must be an instance of Element",
    );
  }, "commandForElement throws error on assignment of non Element");

  test(function () {
    invoker.setAttribute("command", "");
    assert_equals(invoker.getAttribute("command"), "");
    assert_equals(invoker.command, "");
  }, "command reflects '' when attribute empty, setAttribute version");

  test(function () {
    invoker.command = "fooBarBaz";
    assert_equals(invoker.getAttribute("command"), "fooBarBaz");
    assert_equals(invoker.command, "");
  }, "command reflects correctly for invalid");

  test(function () {
    invoker.command = "";
    assert_equals(invoker.getAttribute("command"), "");
    assert_equals(invoker.command, "");
  }, "command reflects '' when attribute empty, IDL version");

  test(function () {
    invoker.command = [1, 2, 3];
    assert_equals(invoker.getAttribute("command"), "1,2,3");
    assert_equals(invoker.command, "");
  }, "command reflects correctly for invalid when array");

  test(function () {
    invoker.command = [];
    assert_equals(invoker.getAttribute("command"), "");
    assert_equals(invoker.command, "");
  }, "command reflects '' when attribute set to []");

  test(function () {
    invoker.command = {};
    assert_equals(invoker.command, "");
  }, "command reflects correctly for invalid when object");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/interface.html"
}
```
