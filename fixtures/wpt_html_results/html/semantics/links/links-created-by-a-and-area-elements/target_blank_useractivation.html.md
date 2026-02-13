# html/semantics/links/links-created-by-a-and-area-elements/target_blank_useractivation.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/links-created-by-a-and-area-elements/target_blank_useractivation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset=utf-8>
<title>Test that clicking target=_blank links consumes userActivation</title>
<link rel="help" href="https://html.spec.whatwg.org/#following-hyperlinks-2">
<link rel="help" href="https://html.spec.whatwg.org/#the-rules-for-choosing-a-navigable">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
</head>
<body>
<a target="_blank" id="a_click_target">Click me</a>
<map id="map">
<area coords="0,0,50,50" target="_blank">
</map>
<img src="/images/blue.png" usemap="#map" style="width: 50px; height: 50px" id="area_click_target">
<script>

const testUrl = "support/target-blank-useractivation.html";

function waitForNewWindow(aBroadcastChannelId) {
  return new Promise(resolve => {
    let channel = new BroadcastChannel(aBroadcastChannelId);
    channel.addEventListener("message", (e) => {
      assert_equals(e.data, "ready");
      channel.postMessage("close");
      resolve();
    }, {once: true});
  });
}

["a", "area"].forEach(tag => {
  const link = document.querySelector(tag);

  promise_test(async () => {
    let broadcastChannelId = `${tag}_click_script`;
    link.href = `${testUrl}?${broadcastChannelId}`;

    await test_driver.bless('transient activation');
    assert_true(navigator.userActivation.isActive, 'should have user activation');

    let newWindowPromise = waitForNewWindow(broadcastChannelId);
    link.click();
    await newWindowPromise;
    assert_false(navigator.userActivation.isActive, 'navigator.userActivation.isActive after opening a new window');
  }, `<${tag} target=_blank">.click()`);

  promise_test(async () => {
    let broadcastChannelId = `${tag}_click`;
    link.href = `${testUrl}?${broadcastChannelId}`;

    let newWindowPromise = waitForNewWindow(broadcastChannelId);
    link.addEventListener("click", () => {
      assert_true(navigator.userActivation.isActive, 'should have user activation');
    });
    // test_driver.click() doesn't work with <area> element in Chrome.
    await new test_driver.Actions()
        .pointerMove(1, 1, { origin: document.getElementById(`${tag}_click_target`) })
        .pointerDown()
        .pointerUp()
        .send();
    await newWindowPromise;
    assert_false(navigator.userActivation.isActive, 'navigator.userActivation.isActive after opening a new window');
  }, `<${tag} target=_blank"> mouse click`);
});
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 791,
        "byte_start": 692,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 791,
        "byte_start": 692,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/links/links-created-by-a-and-area-elements/target_blank_useractivation.html"
}
```
