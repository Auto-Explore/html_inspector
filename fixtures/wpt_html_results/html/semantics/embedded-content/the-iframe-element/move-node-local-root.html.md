# html/semantics/embedded-content/the-iframe-element/move-node-local-root.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/move-node-local-root.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Node moves to another document</title>
<link rel="author" title="Dominic Farolino" href="mailto:dom@chromium.org">
<link rel="help" href="https://crbug.com/40277823">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>

<body>
<button id=button></button>
<script>
// This is a regression test for a Chromium crash: https://crbug.com/40277823.
// The test is reproducible by:
//   1. Creating a node with an event listener for an event type that the
//      compositor cares about; `touchmove` in this case.
//   2. Adopting that node into a tree with a *different* local root (i.e., a
//      tree where the root is a local frame, different from this document, with
//      a remote parent).
//   3. Maintaining a reference to the node that now exists in a different local
//      frame root.
//   4. Add a same-type event listener to the document that used to host the
//      now-adopted node. This fails an assertion in the event handler
//      registry's consistency checker, which is mistakenly holding a reference
//      to the node that is now hosted in a different local frame root, which
//      the checker does not expect.
promise_test(async t => {
  const crossOriginChild = document.createElement('iframe');
  const crossOriginChildURL = new URL('resources/cross-origin-middle-frame.html', get_host_info().HTTP_REMOTE_ORIGIN + location.pathname);
  crossOriginChild.src = crossOriginChildURL;

  const grandchildLoadPromise = new Promise(resolve => {
    window.onmessage = e => {
      if (e.data === 'grandchild loaded') {
        resolve();
      }
    }
  });
  document.body.append(crossOriginChild);
  await grandchildLoadPromise;

  const sameOriginGrandchild = window.frames[0][0];
  assert_not_equals(sameOriginGrandchild.document, null,
      "same-origin grandchild frame exists");

  button.addEventListener('touchmove', e => {});

  // This is important because before https://crbug.com/40277823 was fixed, it
  // would prevent the garbage collector from removing `button` from this
  // document's event handler registry. As long as it's still (incorrectly) in
  // the registry when we add the `touchmove` event handler is added to this
  // document later (post-adoption), the registry's consistency checker would
  // crash, asserting that the still-tracked event target is rooted at its *old*
  // local frame root.
  window.buttonHolder = button;

  sameOriginGrandchild.document.adoptNode(button);
  // This below would previously cause the Chromium crash.
  document.body.addEventListener('touchmove', e => {});
}, "Event handler-bearing node moved across local roots in the same tab/page");
</script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/move-node-local-root.html"
}
```
