# html/syntax/parsing/doctype-system-identifier-distinction.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/doctype-system-identifier-distinction.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>DOCTYPE system identifier: empty string treated as missing</title>
<link
  rel="help"
  href="https://html.spec.whatwg.org/multipage/parsing.html#the-initial-insertion-mode"
/>
<link rel="help" href="https://github.com/whatwg/html/issues/12023" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  // Tests that an empty string system identifier is treated the same as a missing
  // system identifier for quirks mode determination.
  // See https://github.com/whatwg/html/issues/12023
  //
  // The important distinction that must be preserved:
  // - missing system ID → quirks mode (BackCompat)
  // - empty system ID → quirks mode (BackCompat) [same as missing]
  // - non-empty system ID → limited-quirks mode (CSS1Compat) [different from missing]

  const testCases = [
    // HTML 4.01 Frameset - empty system ID (treated same as missing)
    {
      file: "support/doctype-system-id-empty-frameset.html",
      expected: "BackCompat",
      description:
        'DOCTYPE with "-//W3C//DTD HTML 4.01 Frameset//" and empty system ID triggers quirks mode',
    },
    // HTML 4.01 Frameset - missing system ID
    {
      file: "support/doctype-system-id-missing-frameset.html",
      expected: "BackCompat",
      description:
        'DOCTYPE with "-//W3C//DTD HTML 4.01 Frameset//" and missing system ID triggers quirks mode',
    },
    // HTML 4.01 Frameset - non-empty system ID (must remain different!)
    {
      file: "support/doctype-system-id-nonempty-frameset.html",
      expected: "CSS1Compat",
      description:
        'DOCTYPE with "-//W3C//DTD HTML 4.01 Frameset//" and non-empty system ID triggers limited-quirks mode',
    },
    // HTML 4.01 Transitional - empty system ID (treated same as missing)
    {
      file: "support/doctype-system-id-empty-transitional.html",
      expected: "BackCompat",
      description:
        'DOCTYPE with "-//W3C//DTD HTML 4.01 Transitional//" and empty system ID triggers quirks mode',
    },
    // HTML 4.01 Transitional - missing system ID
    {
      file: "support/doctype-system-id-missing-transitional.html",
      expected: "BackCompat",
      description:
        'DOCTYPE with "-//W3C//DTD HTML 4.01 Transitional//" and missing system ID triggers quirks mode',
    },
    // HTML 4.01 Transitional - non-empty system ID (must remain different!)
    {
      file: "support/doctype-system-id-nonempty-transitional.html",
      expected: "CSS1Compat",
      description:
        'DOCTYPE with "-//W3C//DTD HTML 4.01 Transitional//" and non-empty system ID triggers limited-quirks mode',
    },
  ];

  for (const testCase of testCases) {
    async_test((t) => {
      const iframe = document.createElement("iframe");
      iframe.onload = t.step_func_done(() => {
        assert_equals(
          iframe.contentDocument.compatMode,
          testCase.expected,
          `compatMode should be ${testCase.expected}`
        );
        iframe.remove();
      });
      iframe.src = testCase.file;
      document.body.appendChild(iframe);
    }, testCase.description);
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
  "source_name": "html/syntax/parsing/doctype-system-identifier-distinction.html"
}
```
