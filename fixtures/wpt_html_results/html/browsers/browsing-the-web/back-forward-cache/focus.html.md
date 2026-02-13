# html/browsers/browsing-the-web/back-forward-cache/focus.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/focus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta name="timeout" content="long">
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#focused-area-of-the-document">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="resources/helper.sub.js"></script>
<script>
// Focus should remain the same and thus blur/focus events shouldn't be fired
// when page gets into and out of BFCache, as explicitly noted in the spec:
// https://html.spec.whatwg.org/multipage/interaction.html#focused-area-of-the-document
// "Even if a document is not fully active and not shown to the user, it can still
// have a focused area of the document. If a document's fully active state changes,
// its focused area of the document will stay the same."
runBfcacheTest({
  openFunc: (url) => window.open(url + '&events=pagehide,pageshow,load',
                               '_blank', 'noopener'),
  funcBeforeNavigation: () => {
    // Create and focus on an <input> before navigation.
    // Focus/blur events on the <input> are recorded.
    const textInput = document.createElement('input');
    textInput.setAttribute('type', 'text');
    textInput.setAttribute('id', 'toBeFocused');
    textInput.onfocus = () => {
      recordEvent('input.focus');
    };
    textInput.onblur = () => {
      recordEvent('input.blur');
    };
    document.body.appendChild(textInput);
    textInput.focus();
    window.activeElementBeforePageHide = document.activeElement;
    window.addEventListener('pagehide', () => {
      window.activeElementOnPageHide = document.activeElement;
    });
  },
  funcAfterAssertion: async (pageA) => {
    assert_true(
      await pageA.execute_script(() => {
          return window.activeElementBeforePageHide ===
                 document.querySelector('#toBeFocused');
      }),
      'activeElement before pagehide');

    assert_true(
      await pageA.execute_script(() => {
          return window.activeElementOnPageHide ===
                 document.querySelector('#toBeFocused');
      }),
      'activeElement on pagehide');

    assert_true(
      await pageA.execute_script(() => {
          return document.activeElement ===
                 document.querySelector('#toBeFocused');
      }),
      'activeElement after navigation');

    assert_array_equals(
      await pageA.execute_script(() => getRecordedEvents()),
      [
        'window.load',
        'window.pageshow',
        'input.focus',
        'window.pagehide.persisted',
        'window.pageshow.persisted'
      ],
      'blur/focus events should not be fired ' +
      'when page gets into and out of BFCache');
  }
}, 'Focus should be kept when page gets into and out of BFCache');
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/focus.html"
}
```
