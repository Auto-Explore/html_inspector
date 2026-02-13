# html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-wrapped.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-wrapped.html",
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
    <title>HTML Test: BDI: neutral when wrapped</title>
    <link rel="match" href="bdi-neutral-wrapped-ref.html">
    <link rel="author" title="Aharon Lanin" href="mailto:aharon@google.com"/>
    <link rel="author" title="HTML5 bidi test WG" href="mailto:html5bidi@googlegroups.com"/>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-bdi-element"/>
    <meta name="assert" content="
      'For the purposes of applying the bidirectional algorithm to the paragraph-level
      container that a bdi element finds itself within, the bdi element must be treated
      like a U+FFFC OBJECT REPLACEMENT CHARACTER.'
      This should hold even if the BDI's content is wrapped over more than one line."/>
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
        &#xA0; - Non-breaking space.
        &#x05D0; ... &#x05D5; - The first six Hebrew letters (strongly RTL).
        &#x202D; - The LRO (left-to-right override) formatting character.
        &#x202C; - The PDF (pop directional formatting) formatting character; closes LRO.
      In the test below, the non-breaking spaces in the BDI's middle "word" make it so long that it
      must be displayed on a line of its own, with the BDI wrapped before and after it. At the same
      time, the content surrounding the BDI is supposed to form a single directional run, despite
      the containing element and the BDI both having the opposite direction, because the BDI must be
      treated as a neutral. Thus, on the line containing the first part of the BDI, the BDI's
      content must appear after the content preceding it, and on the line containing the last part
      of the BDI, the BDI content must appear before the content following it, where both 'before'
      and 'after' are defined relative to the surrounding directional run.
    </div>
    <div class="test">
      <div dir="ltr">
        &#x05D0; &gt;
        <bdi>b
&gt;&gt;&gt;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;
          c</bdi>
        &gt; &#x05D3;...
      </div>
      <div dir="rtl">
        a &gt;
        <bdi>&#x05D1;
&gt;&gt;&gt;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;&#xA0;
          &#x05D2;</bdi>
        &gt; d...
      </div>
    </div>
    <div class="ref">
      <div dir="ltr">
        &#x202D;b &lt; &#x05D0;&#x202C;<br/>
        &#x202D;&gt;&gt;&gt;&#x202C;<br/>
        &#x202D;&#x05D3; &lt; c...&#x202C;
      </div>
      <div dir="rtl">
        &#x202D;a &gt; &#x05D1;&#x202C;<br/>
        &#x202D;&lt;&lt;&lt;&#x202C;<br/>
        &#x202D;...&#x05D2; &gt; d&#x202C;
      </div>
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
  "source_name": "html/semantics/text-level-semantics/the-bdi-element/bdi-neutral-wrapped.html"
}
```
