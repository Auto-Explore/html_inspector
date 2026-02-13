# html/browsers/the-window-object/window-open-popup-behavior.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/window-open-popup-behavior.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>Window.open popup behavior</title>
<link rel="author" href="masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/window-object.html#window-open-steps">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
function testOne(windowFeatures, expectPopup) {
  const windowName = Math.round(Math.random()*1e12);
  const channel = new BroadcastChannel(windowName);
  var w;
  promise_test(() => {
    return new Promise(resolve => {
      w = window.open("support/window-open-popup-target.html", windowName, windowFeatures);
      channel.addEventListener('message', resolve);
    }).then(e => {
      // Send message first so if asserts throw the popup is still closed
      channel.postMessage(null);
      assert_false(e.data.mixedState, "No mixed state");
      assert_equals(e.data.isPopup, expectPopup, "Popup state");
    });
  },`${windowFeatures} (expect ${expectPopup ? "popup" : "tab"})`);
}

// No windowpreferences at all - tab.
testOne(undefined, /*expectPopup=*/false);

// Test all permutations of these properties:
const features = ["location","toolbar","menubar","resizable","scrollbars","status"];
const nProps = features.length;
const skip = 7; // To speed up the test, don't test all values. Skip 7 to pseudo-randomize.
for(let i=0;i<2**nProps;i+=skip) {
  const enableVec = Number(i).toString(2).padStart(nProps,'0').split('').map(s => (s==="1"));
  let windowFeatures = [];
  for(let i=0;i<nProps;++i) {
    if (enableVec[i])
      windowFeatures.push(features[i] + "=yes");
  }
  windowFeatures = windowFeatures.join(',');
  // We get a popup we got windowFeatures, and any of them are false:
  const expectPopup = !!windowFeatures.length && (!(enableVec[0] || enableVec[1]) || !enableVec[2] || !enableVec[3] || !enableVec[4] || !enableVec[5]);
  testOne(windowFeatures, expectPopup);
  testOne(windowFeatures + ",noopener", /*expectPopup=*/false);
  testOne(windowFeatures + ",noreferrer", /*expectPopup=*/false);
  testOne(windowFeatures + ",popup", /*expectPopup=*/true); // "popup" feature = popup
  testOne(windowFeatures + ",noopener,noreferrer,popup", /*expectPopup=*/false);
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
  "source_name": "html/browsers/the-window-object/window-open-popup-behavior.html"
}
```
