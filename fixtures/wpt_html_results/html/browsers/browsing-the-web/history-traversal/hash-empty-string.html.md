# html/browsers/browsing-the-web/history-traversal/hash-empty-string.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/hash-empty-string.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help"
      href="https://html.spec.whatwg.org/multipage/nav-history-apis.html#the-location-interface:concept-url-fragment-4">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1544428">
<link rel="author" title="Zach Hoffman" href="mailto:zach@zrhoffman.net">
<script>
let popstateTriggered = false;
window.addEventListener("popstate", () => popstateTriggered = true);

let hashchangeTriggered = false;
window.addEventListener("hashchange", () => hashchangeTriggered = true);

test(() => {
  assert_equals(location.href.indexOf("#"), -1)
}, "URL has no hash")

location.hash = "";

test(() => {
  assert_false(popstateTriggered);
}, "changing the hash from an empty string to an empty string does not trigger a popstate event")

promise_test(async () => {
  // hashchange is fired async
  await new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r)));
  assert_false(hashchangeTriggered);
}, "changing the hash from an empty string to an empty string does not trigger a hashchange event")
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/hash-empty-string.html"
}
```
