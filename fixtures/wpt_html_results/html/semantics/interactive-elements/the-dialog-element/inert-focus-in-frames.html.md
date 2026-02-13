# html/semantics/interactive-elements/the-dialog-element/inert-focus-in-frames.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/inert-focus-in-frames.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:falken@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=242848">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe height=400 width=600 id="main-iframe">
  <frameset rows="*" cols="50,50">
    <frame src="resources/inert-focus-in-frames-frame1.html">
    <frame src="resources/inert-focus-in-frames-frame2.html">
  </frameset>
</iframe>

<script>
let framesLoadedResolver = null;
const framesLoadedPromise = new Promise(resolve => framesLoadedResolver = resolve);
framesLoaded = 0;
numFrames = 4;

function frameLoaded() {
  framesLoaded++;
  if (framesLoaded == numFrames)
    framesLoadedResolver();
}
var mainIframe = document.getElementById('main-iframe');
mainIframe.contentDocument.write(mainIframe.textContent);
mainIframe.contentDocument.close();
mainIframe.contentWindow.frames[1].window.onload = frameLoaded;
window.onload = frameLoaded;

promise_test(async () => {
  await framesLoadedPromise;
  // Chrome and edge auto-focus the URL bar when the browser is launched.
  // This is needed to ensure the 'focus' events fire below.
  await test_driver.click(document.documentElement);

  function testFocus(element, expectFocus) {
    let focusedElement = null;
    element.addEventListener('focus', function() { focusedElement = element; }, false);
    element.focus();
    if (expectFocus) {
      assert_equals(focusedElement, element, element.id);
    } else {
      assert_not_equals(focusedElement, element, element.id);
    }
  }

  // Opening a modal dialog in frame1. It blocks other nodes in its document.
  const frame1 = mainIframe.contentWindow.frames[0].document;
  frame1.querySelector('dialog').showModal();

  testFocus(frame1.querySelector('.target'), false);
  const iframe = frame1.querySelector('#iframe1').contentDocument;
  testFocus(iframe.querySelector('.target'), true);

  // Even a modal dialog in the iframe is blocked by the modal dialog in the parent frame1.
  iframe.querySelector('dialog').showModal();
  testFocus(iframe.querySelector('button'), false);

  // An iframe within a modal dialog can still be focused.
  var dialogIframe = frame1.querySelector('#iframe-in-dialog').contentDocument;
  testFocus(dialogIframe.querySelector('.target'), true);

  // A modal dialog does not block nodes in a sibling frame.
  var frame2 = mainIframe.contentWindow.frames[1].document;
  testFocus(frame2.querySelector('.target'), true);

  // Closing the dialog in frame1. The modal dialog in the iframe does not block nodes in its parent.
  frame1.querySelector('dialog').close();
  testFocus(iframe.querySelector('.target'), false);
  testFocus(frame1.querySelector('.target'), true);

}, 'Tests inert node focusing across frames and iframes.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.text.disallowed",
      "message": "Text not allowed in “iframe” in this context.",
      "severity": "Warning",
      "span": null
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/inert-focus-in-frames.html"
}
```
