# html/dom/render-blocking/non-render-blocking-scripts.optional.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/render-blocking/non-render-blocking-scripts.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests when script is not implicitly potentially render-blocking</title>
<link rel="help" href="https://github.com/whatwg/html/pull/7894">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/test-render-blocking.js"></script>

<!--
  The test is marked "optional" because even when the document is not
  render-blocked, the user agent is still free to take other factors, which are
  not limited by the spec, into consideration and therefore decide not to
  render. However, it is still more desirable if rendering starts
  immediately/soon.
-->

<script class="test" data="parser-inserted async script" async
        src="support/dummy-1.js?pipe=trickle(d1)&async"></script>
<script class="test" data="parser-inserted defer script" defer
        src="support/dummy-1.js?pipe=trickle(d1)&defer"></script>
<script class="test" data="parser-inserted module script" type="module"
        src="support/dummy-1.mjs?pipe=trickle(d1)"></script>
<script class="test" data="parser-inserted async module script" type="module"
        async src="support/dummy-1.mjs?pipe=trickle(d1)&async"></script>

<script>
function addTestScriptElement(title, attributes) {
  let element = document.createElement('script');
  element.className = 'test';
  element.setAttribute('data', title);
  Object.assign(element, attributes);
  document.head.appendChild(element);
}

addTestScriptElement('script-inserted script', {src: 'support/dummy-1.js?pipe=trickle(d1)&dynamic'});
addTestScriptElement('script-inserted sync script', {async: false, src: 'support/dummy-1.js?pipe=trickle(d1)&dynamicSync'});
addTestScriptElement('script-inserted module script', {type: 'module', src: 'support/dummy-1.mjs?pipe=trickle(d1)&dynamic'});
</script>

<div id="dummy">Some text</div>

<script>
const testElements = [...document.querySelectorAll('.test')];
const loadObservers = testElements.map(element => new LoadObserver(element));

promise_setup(async () => {
  // Test cases are run after rendering is unblocked.
  await new Promise(resolve => requestAnimationFrame(resolve));
});

for (let index = 0; index < testElements.length; ++index) {
  promise_test(
    async () => assert_false(loadObservers[index].finished),
    testElements[index].getAttribute('data') + ' is not implicitly render-blocking');
}

for (let index = 0; index < testElements.length; ++index) {
  promise_test(
    () => loadObservers[index].load,
    testElements[index].getAttribute('data') + ' should eventually be loaded and evaluated');
}
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
  "source_name": "html/dom/render-blocking/non-render-blocking-scripts.optional.html"
}
```
