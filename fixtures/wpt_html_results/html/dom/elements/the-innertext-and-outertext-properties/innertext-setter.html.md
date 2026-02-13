# html/dom/elements/the-innertext-and-outertext-properties/innertext-setter.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/the-innertext-and-outertext-properties/innertext-setter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>innerText setter test</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="container"></div>
<script>
// As of March 2017, WebKit and Blink have inconsistent results depending on
// rendered or not.  setupTest() tests a rendered case, and setupTestDetached()
// tests a not-rendered case.

function setupTest(context, plain) {
  var container = document.getElementById("container");
  // context is either a string or an element node
  if (typeof context === "string") {
    container.innerHTML = context;
  } else {
    container.innerHTML = "";
    container.appendChild(context);
  }
  var e = container.firstChild;
  while (e && e.nodeType != Node.ELEMENT_NODE) {
    e = e.nextSibling;
  }
  e.offsetWidth;
  var oldChild = e.firstChild;
  e.innerText = plain;
  return [e, oldChild];
}

function setupTestDetached(context, plain) {
  var detachedContainer = document.createElement("div");
  // context is either a string or an element node
  if (typeof context === "string") {
    detachedContainer.innerHTML = context;
  } else {
    detachedContainer.innerHTML = "";
    detachedContainer.appendChild(context);
  }
  var e = detachedContainer.firstChild;
  while (e && e.nodeType != Node.ELEMENT_NODE) {
    e = e.nextSibling;
  }
  var oldChild = e.firstChild;
  e.innerText = plain;
  return [e, oldChild];
}

function assertNewSingleTextNode(newChild, expectedText, oldChild) {
  assert_not_equals(newChild, null, "Should have a child");
  assert_equals(newChild.nodeType, Node.TEXT_NODE, "Child should be a text node");
  assert_equals(newChild.nextSibling, null, "Should have only one child");
  assert_equals(newChild.data, expectedText);
  assert_not_equals(newChild, oldChild, "Child should be a *new* text node");
}

function assertNoEmptyTextChild(parent) {
  for (var child = parent.firstChild; child; child = child.nextSibling) {
    if (child.nodeType === Node.TEXT_NODE) {
      assert_not_equals(child.data, "", "Should not have empty text nodes");
    }
  }
}

function testText(context, plain, expectedText, msg) {
  test(function(){
    var arr = setupTest(context, plain);
    assertNewSingleTextNode(arr[0].firstChild, expectedText, arr[1]);
  }, msg);
  test(function() {
    var arr = setupTestDetached(context, plain);
    assertNewSingleTextNode(arr[0].firstChild, expectedText, arr[1]);
  }, msg + ", detached");
}

function testHTML(context, plain, expectedHTML, msg) {
  test(function(){
    var e = setupTest(context, plain)[0];
    assert_equals(e.innerHTML, expectedHTML);
    assertNoEmptyTextChild(e);
  }, msg);
  test(function() {
    var e = setupTestDetached(context, plain)[0];
    assert_equals(e.innerHTML, expectedHTML);
    assertNoEmptyTextChild(e);
  }, msg + ", detached");
}
</script>
<script src="innertext-setter-tests.js"></script>
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
  "source_name": "html/dom/elements/the-innertext-and-outertext-properties/innertext-setter.html"
}
```
