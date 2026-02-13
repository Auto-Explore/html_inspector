# html/semantics/scripting-1/the-script-element/change-src-attr-prepare-a-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/change-src-attr-prepare-a-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<link rel=help href=https://github.com/whatwg/html/pull/10188#discussion_r1719338657>
<title>Adding/changing src attribute does "prepare the script"</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
// The "old" HTML Standard specification text around `src` attribute mutation
// for non-parser-inserted scripts *ONLY* "prepared"/run the script iff the src
// attribute "previously had no such attribute". This changed in
// https://github.com/whatwg/html/pull/10188 to align with a majority of
// browsers. This test ensures that `src` mutations on these kinds of scripts
// *where a previous valid `src` attribute existed* do indeed "prepare" the
// script.
promise_test(async () => {
  window.didExecute = false;

  const script = document.createElement('script');
  // Invalid type, so the script won't execute upon insertion.
  script.type = 'invalid';
  script.src = 'resources/flag-setter.js';
  document.body.append(script);
  assert_false(window.didExecute);

  // Make script valid, but don't immediately execute it.
  script.type = '';

  const scriptPromise = new Promise(resolve => {
    script.onload = resolve;
  });

  // Mutating the `src` attribute, which has an existing valid value, triggers
  // the "prepare a script" algorithm via the post-connection steps.
  script.src = 'resources/flag-setter.js?different';

  await scriptPromise;
  assert_true(window.didExecute);
}, "Mutating `src` attribute from an already-valid value does 'prepare' the script");
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
  "source_name": "html/semantics/scripting-1/the-script-element/change-src-attr-prepare-a-script.html"
}
```
