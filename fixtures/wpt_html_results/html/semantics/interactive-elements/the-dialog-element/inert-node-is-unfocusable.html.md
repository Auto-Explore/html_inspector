# html/semantics/interactive-elements/the-dialog-element/inert-node-is-unfocusable.html

Counts:
- errors: 2
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-unfocusable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html id="html" tabindex="1">
<head>
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#blocked-by-a-modal-dialog">
<meta name="assert" content="Checks that, when opening modal dialogs, inert nodes are not focusable.">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body id="body" tabindex="1">
<style>
dialog {
  outline: none;
}
</style>
<dialog id="top-dialog" tabindex="1" style="width: 100px; top: 30px"><button id="top-dialog-button">I get focus</button></dialog>
<dialog id="bottom-dialog" tabindex="-1" style="width: 100px; bottom: 30px"><button id="bottom-dialog-button">I don't get focus.</button></dialog>
<div id="container">
    <input id="text" type="text">
    <input id="datetime" type="datetime">
    <input id="color" type="color">
    <select id="select">
        <optgroup id="optgroup">
            <option id="option">Option</option>
        </optgroup>
    </select>
    <div id="contenteditable-div" contenteditable>I'm editable</div>
    <span id="tabindex-span" tabindex="0">I'm tabindexed.</div>
    <embed id="embed" type="application/x-blink-test-plugin" width=100 height=100></embed>
    <a id="anchor" href="">Link</a>
</div>
<script>
"use strict";
// The test passses if only the topmost dialog and its button are focusable.

function testFocus(element, expectFocus) {
    promise_test(async function() {
        var focusedElement = null;
        element.addEventListener('focus', function() { focusedElement = element; }, false);
        element.focus();
        var theElement = element;
        if (expectFocus) {
            assert_equals(focusedElement, theElement);
        } else {
            assert_not_equals(focusedElement, theElement);
        }
    }, `#${CSS.escape(element.id)} is ${expectFocus ? "" : "not "} focusable`);
}

function testTree(element, expectFocus) {
    if (element.nodeType == Node.ELEMENT_NODE)
        testFocus(element, expectFocus);
    var childNodes = element.childNodes;
    for (var i = 0; i < childNodes.length; i++)
        testTree(childNodes[i], expectFocus);
}

var bottomDialog = document.getElementById('bottom-dialog');
var topDialog = document.getElementById('top-dialog');
promise_setup(async function() {
    await test_driver.click(document.documentElement);
    bottomDialog.showModal();
    topDialog.showModal();
    add_completion_callback(function() {
        topDialog.close();
        bottomDialog.close();
    });
});

testFocus(document.documentElement, false);
testFocus(document.body, false);
testTree(topDialog, true);
testTree(bottomDialog, false);
testTree(document.getElementById('container'), false);
</script>
</body>
</html>
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
        "byte_end": 519,
        "byte_start": 512,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 1330,
        "byte_start": 1322,
        "col": 83,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 1373,
        "byte_start": 1367,
        "col": 1,
        "line": 32
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-node-is-unfocusable.html"
}
```
