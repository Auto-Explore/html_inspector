# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/nav-cancelation-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/nav-cancelation-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent main frame cancels a same-origin child whose navigation is pending</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  This test asserts that a parent canceling a same-origin child's cross-origin
  navigation does not result in load events firing synchronously in the parent
-->

<body>

<iframe src=resources/slow.py></iframe>

<script>
promise_test(async t => {
  let window_load_fired = false;
  let iframe_load_fired = false;
  const iframe = document.querySelector('iframe');

  const window_load_promise = new Promise(resolve => {
    window.onload = () => {
      window_load_fired = true;
      resolve();
    }
  });

  const iframe_onload_promise = new Promise(resolve => {
    iframe.onload = () => {
      iframe_load_fired = true;
      resolve();
    }
  });

  // While the child navigation is in-flight, cancel it and record when the
  // parent `load` event fires.
  window.frames[0].location.href = "resources/slow.py?different";

  // Synchronously after cancelation, no load events should have been fired.
  assert_false(window_load_fired,
    "Parent's load event does not synchronously fire after cancelation");
  assert_false(iframe_load_fired,
    "<iframe> load event does not synchronously fire after cancelation");

  // Load events did not fire in a microtask after cancelation.
  await Promise.resolve();
  assert_false(window_load_fired,
      "Parent's load event does not fire in the microtask after cancelation");
  assert_false(iframe_load_fired,
      "<iframe> load event does not fire in the microtask after cancelation");

  // Canceling the navigation should unblock the parent's load event, but the
  // new iframe navigation should still be pending, and the iframe load event
  // shouldn't fire until *that one* is complete.
  await window_load_promise;
  assert_true(window_load_fired,
      "Parent's load event fires asynchronously after child navigation cancelation");
  assert_false(iframe_load_fired,
      "<iframe> load event does not fire until subsequent navigation is complete");
}, "parent cancels a pending navigation in a same-origin child");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/nav-cancelation-1.html"
}
```
