# html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-to-surrounding-run.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-to-surrounding-run.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8"/>
    <title>HTML Test: BDI: neutral to surrounding letters</title>
    <link rel="match" href="bdi-neutral-to-surrounding-run-ref.html">
    <link rel="author" title="Aharon Lanin" href="mailto:aharon@google.com"/>
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com"/>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-bdi-element"/>
    <meta name="assert" content="
      'For the purposes of applying the bidirectional algorithm to the paragraph-level
      container that a bdi element finds itself within, the bdi element must be treated
      like a U+FFFC OBJECT REPLACEMENT CHARACTER.'
      Thus, regardless of its content and its dir attribute (if any), a BDI will not prevent
      a strongly RTL (or LTR) character preceding it from forming a single directional run with
      another strongly RTL (LTR) character following it."/>
    <style>
      body{
        font-size:2em;
      }
      .test, .ref {
        border: medium solid gray;
        width: 400px;
        margin: 20px;
      }
      .comments { display: none; }
    </style>
  </head>
  <body>
    <div class="instructions"><p>Test passes if the two boxes below look exactly the same.</p></div>
    <div class="comments">
      Key to entities used below:
        &#x05D0; ... &#x05D5; - The first six Hebrew letters (strongly RTL).
        &#x202D; - The LRO (left-to-right override) formatting character.
        &#x202C; - The PDF (pop directional formatting) formatting character; closes LRO.
      If the BDI in the test's first DIV were a SPAN, its b would prevent the &#x05D0; and the &#x05D1;
      from forming a single RTL run and thus keep the &gt;s between from being mirrored into &lt;s.
    </div>
    <div class="test">
      <div dir="ltr">&#x05D0; &gt; <bdi>[b]</bdi> &gt; &#x05D2;...</div>
      <div dir="ltr">&#x05D0; &gt; <bdi dir="ltr">[b]</bdi> &gt; &#x05D2;...</div>
      <div dir="ltr">&#x05D0; &gt; <bdi dir="rtl">[b]</bdi> &gt; &#x05D2;...</div>
      <div dir="rtl">a &gt; <bdi>[&#x05D1;]</bdi> &gt; c...</div>
      <div dir="rtl">a &gt; <bdi dir="ltr">[&#x05D1;]</bdi> &gt; c...</div>
      <div dir="rtl">a &gt; <bdi dir="rtl">[&#x05D1;]</bdi> &gt; c...</div>
    </div>
    <div class="ref">
      <div dir="ltr">&#x202D;&#x05D2; &lt; [b] &lt; &#x05D0;...&#x202C;</div>
      <div dir="ltr">&#x202D;&#x05D2; &lt; [b] &lt; &#x05D0;...&#x202C;</div>
      <div dir="ltr">&#x202D;&#x05D2; &lt; [b] &lt; &#x05D0;...&#x202C;</div>
      <div dir="rtl">&#x202D;...a &gt; [&#x05D1;] &gt; c&#x202C;</div>
      <div dir="rtl">&#x202D;...a &gt; [&#x05D1;] &gt; c&#x202C;</div>
      <div dir="rtl">&#x202D;...a &gt; [&#x05D1;] &gt; c&#x202C;</div>
    </div>
  </body>
</html>
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
  "source_name": "html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-to-surrounding-run.html"
}
```
