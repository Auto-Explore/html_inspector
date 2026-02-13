# html/semantics/scripting-1/the-script-element/module/modulepreload-referrerpolicy.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/modulepreload-referrerpolicy.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Modulepreload Referrer Policy Tests</title>
<meta name="author" title="Google" href="https://www.google.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/links.html#link-type-modulepreload">
<link rel="help" href="https://w3c.github.io/webappsec-referrer-policy/">
<meta name="assert" content="link rel=modulepreload respects the referrerpolicy attribute">
<!--
  This test verifies that modulepreload correctly handles various referrer policies
  for same-origin requests. It tests all standard referrer policy values:
  - no-referrer
  - origin
  - same-origin
  - strict-origin
  - strict-origin-when-cross-origin
  - unsafe-url
  Each policy is tested by creating a modulepreload link dynamically with the
  specific policy and verifying the referrer header that was sent.
-->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
// Initialize the window.referrers object that will be used by echo-referrer.py
window.referrers = {};
</script>
</head>
<body>
<script>
// Helper function to create a unique ID for each test
function generateUniqueId() {
  return Date.now() + Math.floor(Math.random() * 1000);
}

// Helper function to create a modulepreload element with a given referrer policy
function createModulePreload(url, referrerPolicy = null) {
  const link = document.createElement('link');
  link.rel = 'modulepreload';
  link.href = url;

  if (referrerPolicy !== null) {
    link.referrerPolicy = referrerPolicy;
  }

  return link;
}

// Helper function to wait for preload completion
function waitForPreload(link) {
  return new Promise((resolve, reject) => {
    link.onload = resolve;
    link.onerror = () => reject(new Error("Modulepreload failed to load"));
  });
}

// Test default referrer policy behavior
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // First import to establish baseline
  await import(url);

  // Create modulepreload
  const link = createModulePreload(url);
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // Check if referrer was sent
  assert_equals(window.referrers[uid], location.href, "Modulepreload should use default referrer policy");

}, "Modulepreload should use default referrer policy");

// Test explicit no-referrer policy
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // Create modulepreload with no-referrer policy
  const link = createModulePreload(url, 'no-referrer');
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // Check if referrer was NOT sent
  assert_equals(window.referrers[uid], "", "Modulepreload with no-referrer policy should not send referrer");

}, "Modulepreload with no-referrer policy should not send referrer");

// Test origin referrer policy
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // Create modulepreload with origin policy
  const link = createModulePreload(url, 'origin');
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // Check if origin-only referrer was sent
  assert_equals(window.referrers[uid], location.origin + "/", "Modulepreload with origin policy should send origin-only referrer");

}, "Modulepreload with origin policy should send origin-only referrer");

// Test same-origin referrer policy
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // Create modulepreload with same-origin policy
  const link = createModulePreload(url, 'same-origin');
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // Check if full referrer was sent (for same-origin requests)
  assert_equals(window.referrers[uid], location.href, "Modulepreload with same-origin policy should send full referrer for same-origin requests");

}, "Modulepreload with same-origin policy should send full referrer for same-origin requests");

// Test strict-origin referrer policy
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // Create modulepreload with strict-origin policy
  const link = createModulePreload(url, 'strict-origin');
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // Check if origin-only referrer was sent
  assert_equals(window.referrers[uid], location.origin + "/", "Modulepreload with strict-origin policy should send origin-only referrer");

}, "Modulepreload with strict-origin policy should send origin-only referrer");

// Test strict-origin-when-cross-origin referrer policy
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // Create modulepreload with strict-origin-when-cross-origin policy
  const link = createModulePreload(url, 'strict-origin-when-cross-origin');
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // For same-origin requests, full URL should be sent
  assert_equals(window.referrers[uid], location.href, "Modulepreload with strict-origin-when-cross-origin policy should send full referrer for same-origin requests");

}, "Modulepreload with strict-origin-when-cross-origin policy should send full referrer for same-origin requests");

// Test unsafe-url referrer policy
promise_test(async t => {
  const uid = generateUniqueId();
  const url = `/preload/resources/echo-referrer.py?uid=${uid}`;

  // Create modulepreload with unsafe-url policy
  const link = createModulePreload(url, 'unsafe-url');
  const preloadComplete = waitForPreload(link);

  document.head.appendChild(link);
  await preloadComplete;

  // Import again to ensure the module is loaded
  await import(url);

  // Check if full referrer was sent
  assert_equals(window.referrers[uid], location.href, "Modulepreload with unsafe-url policy should send full referrer");

}, "Modulepreload with unsafe-url policy should send full referrer");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 147,
        "byte_start": 81,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 147,
        "byte_start": 81,
        "col": 1,
        "line": 5
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/modulepreload-referrerpolicy.html"
}
```
