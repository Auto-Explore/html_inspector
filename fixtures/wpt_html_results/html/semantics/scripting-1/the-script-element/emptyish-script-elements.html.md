# html/semantics/scripting-1/the-script-element/emptyish-script-elements.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/emptyish-script-elements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Treatment of various empty-ish script elements</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#prepare-a-script">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#already-started">
<link rel="help" href="https://github.com/whatwg/html/issues/3419">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script id="no-children"></script>
<script id="whitespace-child"> </script>

<script id="gets-a-no-text-child"></script>
<script id="gets-an-empty-text-child"></script>
<script id="gets-a-text-child"></script>
<script id="gets-a-comment-child"></script>
<script id="gets-a-text-descendant"></script>

<script>
"use strict";

test(() => {
  const el = document.querySelector("#no-children");
  el.appendChild(document.createTextNode("window.noChildrenExecuted = true;"));
  assert_true(window.noChildrenExecuted);
}, "A script with no children bails early, before setting already-started, so can be executed when adding text");

test(() => {
  const el = document.querySelector("#whitespace-child");
  el.appendChild(document.createTextNode("window.whitespaceChildExecuted = true;"));
  assert_equals(window.whitespaceChildExecuted, undefined);
}, "A script with a whitespace child executes, setting already-started, so adding text is a no-op");

test(() => {
  const el = document.querySelector("#gets-a-no-text-child");
  el.appendChild(document.createElement("span"));
  el.appendChild(document.createTextNode("window.getsANoTextChildExecuted = true;"));
  assert_true(window.getsANoTextChildExecuted);
}, "A script with an empty element inserted bails early, before setting already-started, so can be executed when adding text");

test(() => {
  const el = document.querySelector("#gets-an-empty-text-child");
  el.appendChild(document.createTextNode(""));
  el.appendChild(document.createTextNode("window.getsAnEmptyTextChildExecuted = true;"));
  assert_true(window.getsAnEmptyTextChildExecuted);
}, "A script with an empty text node inserted bails early, before setting already-started, so can be executed when adding text");

test(() => {
  const el = document.querySelector("#gets-a-text-child");
  el.appendChild(document.createTextNode("window.getsATextChildExecuted1 = true;"));
  el.appendChild(document.createTextNode("window.getsATextChildExecuted2 = true;"));
  assert_true(window.getsATextChildExecuted1);
  assert_equals(window.getsATextChildExecuted2, undefined);
}, "A script with a text child inserted executes, setting already-started, so adding text is a no-op");

test(() => {
  const el = document.querySelector("#gets-a-comment-child");
  el.appendChild(document.createComment("window.getsACommentChild1 = true;"));
  el.appendChild(document.createTextNode("window.getsACommentChild2 = true;"));
  assert_equals(window.getsACommentChild1, undefined);
  assert_true(window.getsACommentChild2);
}, "A script with a comment child inserted bails early, before setting already-started, so can be executed when adding text");

test(() => {
  const el = document.querySelector("#gets-a-text-descendant");
  const child = document.createElement("span");
  child.appendChild(document.createTextNode("window.getsATextDescendantExecuted1 = true;"));
  el.appendChild(child);
  el.appendChild(document.createTextNode("window.getsATextDescendantExecuted2 = true;"));
  assert_equals(window.getsATextDescendantExecuted1, undefined);
  assert_true(window.getsATextDescendantExecuted2);
}, "A script with an element containing text inserted bails early, before setting already-started, so can be executed when adding text");
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
  "source_name": "html/semantics/scripting-1/the-script-element/emptyish-script-elements.html"
}
```
