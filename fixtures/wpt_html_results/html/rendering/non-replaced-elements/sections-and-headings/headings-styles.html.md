# html/rendering/non-replaced-elements/sections-and-headings/headings-styles.html

Counts:
- errors: 0
- warnings: 77
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/sections-and-headings/headings-styles.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>default styles for h1..h6, hgroup, article, aside, nav, section (no h1 in section UA styles)</title>
<meta name="viewport" content="width=device-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/rendering/support/test-ua-stylesheet.js"></script>
<link rel="help" href="https://github.com/whatwg/html/issues/7867">
<style>
/* Specify this bogus namespace, so the rules in this stylesheet only apply to the `fakeClone`d elements in #refs, not the HTML elements in #tests. */
@namespace url(urn:not-html);

article, aside, h1, h2, h3, h4, h5, h6, hgroup, nav, section {
  display: block;
}

h1 { margin-block: 0.67em; font-size: 2.00em; font-weight: bold; }
h2 { margin-block: 0.83em; font-size: 1.50em; font-weight: bold; }
h3 { margin-block: 1.00em; font-size: 1.17em; font-weight: bold; }
h4 { margin-block: 1.33em; font-size: 1.00em; font-weight: bold; }
h5 { margin-block: 1.67em; font-size: 0.83em; font-weight: bold; }
h6 { margin-block: 2.33em; font-size: 0.67em; font-weight: bold; }

</style>

<div id="log"></div>

<div id="tests">
  <h1></h1>
  <h2></h2>
  <h3></h3>
  <h4></h4>
  <h5></h5>
  <h6></h6>
  <hgroup></hgroup>
  <article></article>
  <aside></aside>
  <nav></nav>
  <section></section>
  <article data-skip>
    <h1></h1>
    <article data-skip>
      <h1></h1>
      <article data-skip>
        <h1></h1>
        <article data-skip>
          <h1></h1>
          <article data-skip>
            <h1></h1>
            <hgroup data-skip>
             <h1></h1>
             <h2></h2>
             <h3></h3>
             <h4></h4>
             <h5></h5>
            </hgroup>
          </article>
        </article>
      </article>
    </article>
  </article>
  <aside data-skip>
    <h1></h1>
    <aside data-skip>
      <h1></h1>
      <aside data-skip>
        <h1></h1>
        <aside data-skip>
          <h1></h1>
          <aside data-skip>
            <h1></h1>
            <hgroup data-skip>
             <h1></h1>
             <h2></h2>
             <h3></h3>
             <h4></h4>
             <h5></h5>
            </hgroup>
          </aside>
        </aside>
      </aside>
    </aside>
  </aside>
  <nav data-skip>
    <h1></h1>
    <nav data-skip>
      <h1></h1>
      <nav data-skip>
        <h1></h1>
        <nav data-skip>
          <h1></h1>
          <nav data-skip>
            <h1></h1>
            <hgroup data-skip>
             <h1></h1>
             <h2></h2>
             <h3></h3>
             <h4></h4>
             <h5></h5>
            </hgroup>
          </nav>
        </nav>
      </nav>
    </nav>
  </nav>
  <section data-skip>
    <h1></h1>
    <section data-skip>
      <h1></h1>
      <section data-skip>
        <h1></h1>
        <section data-skip>
          <h1></h1>
          <section data-skip>
            <h1></h1>
            <hgroup data-skip>
             <h1></h1>
             <h2></h2>
             <h3></h3>
             <h4></h4>
             <h5></h5>
            </hgroup>
          </section>
        </section>
      </section>
    </section>
  </section>
</div>

<div id="refs"></div>

<script>
  const props = [
    'display',
    'margin-top',
    'margin-right',
    'margin-bottom',
    'margin-left',
    'padding-top',
    'padding-right',
    'padding-bottom',
    'padding-left',
    'font-size',
    'font-weight',
  ];
  runUAStyleTests(props);

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1157,
        "byte_start": 1152,
        "col": 7,
        "line": 28
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1169,
        "byte_start": 1164,
        "col": 7,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1181,
        "byte_start": 1176,
        "col": 7,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1193,
        "byte_start": 1188,
        "col": 7,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1205,
        "byte_start": 1200,
        "col": 7,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1217,
        "byte_start": 1212,
        "col": 7,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 1259,
        "byte_start": 1249,
        "col": 12,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.section.lacks_heading",
      "message": "Section lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all sections, or else use a “div” element instead for any cases where no heading is needed.",
      "severity": "Warning",
      "span": {
        "byte_end": 1313,
        "byte_start": 1303,
        "col": 12,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1349,
        "byte_start": 1344,
        "col": 9,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1384,
        "byte_start": 1380,
        "col": 7,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1389,
        "byte_start": 1384,
        "col": 11,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1428,
        "byte_start": 1424,
        "col": 9,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1433,
        "byte_start": 1428,
        "col": 13,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1476,
        "byte_start": 1472,
        "col": 11,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1481,
        "byte_start": 1476,
        "col": 15,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1528,
        "byte_start": 1524,
        "col": 13,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1533,
        "byte_start": 1528,
        "col": 17,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1582,
        "byte_start": 1578,
        "col": 14,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1587,
        "byte_start": 1582,
        "col": 18,
        "line": 50
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1610,
        "byte_start": 1605,
        "col": 18,
        "line": 51
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1633,
        "byte_start": 1628,
        "col": 18,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1656,
        "byte_start": 1651,
        "col": 18,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1679,
        "byte_start": 1674,
        "col": 18,
        "line": 54
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 1741,
        "byte_start": 1731,
        "col": 9,
        "line": 57
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 1758,
        "byte_start": 1748,
        "col": 7,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 1773,
        "byte_start": 1763,
        "col": 5,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.article.lacks_heading",
      "message": "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
      "severity": "Warning",
      "span": {
        "byte_end": 1786,
        "byte_start": 1776,
        "col": 3,
        "line": 60
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1820,
        "byte_start": 1815,
        "col": 9,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1853,
        "byte_start": 1849,
        "col": 7,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1858,
        "byte_start": 1853,
        "col": 11,
        "line": 64
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1895,
        "byte_start": 1891,
        "col": 9,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1900,
        "byte_start": 1895,
        "col": 13,
        "line": 66
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1941,
        "byte_start": 1937,
        "col": 11,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1946,
        "byte_start": 1941,
        "col": 15,
        "line": 68
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1991,
        "byte_start": 1987,
        "col": 13,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 1996,
        "byte_start": 1991,
        "col": 17,
        "line": 70
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2045,
        "byte_start": 2041,
        "col": 14,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2050,
        "byte_start": 2045,
        "col": 18,
        "line": 72
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2073,
        "byte_start": 2068,
        "col": 18,
        "line": 73
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2096,
        "byte_start": 2091,
        "col": 18,
        "line": 74
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2119,
        "byte_start": 2114,
        "col": 18,
        "line": 75
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2142,
        "byte_start": 2137,
        "col": 18,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2271,
        "byte_start": 2266,
        "col": 9,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2302,
        "byte_start": 2298,
        "col": 7,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2307,
        "byte_start": 2302,
        "col": 11,
        "line": 86
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2342,
        "byte_start": 2338,
        "col": 9,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2347,
        "byte_start": 2342,
        "col": 13,
        "line": 88
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2386,
        "byte_start": 2382,
        "col": 11,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2391,
        "byte_start": 2386,
        "col": 15,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2434,
        "byte_start": 2430,
        "col": 13,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2439,
        "byte_start": 2434,
        "col": 17,
        "line": 92
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2488,
        "byte_start": 2484,
        "col": 14,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2493,
        "byte_start": 2488,
        "col": 18,
        "line": 94
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2516,
        "byte_start": 2511,
        "col": 18,
        "line": 95
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2539,
        "byte_start": 2534,
        "col": 18,
        "line": 96
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2562,
        "byte_start": 2557,
        "col": 18,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2585,
        "byte_start": 2580,
        "col": 18,
        "line": 98
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2708,
        "byte_start": 2703,
        "col": 9,
        "line": 106
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2743,
        "byte_start": 2739,
        "col": 7,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2748,
        "byte_start": 2743,
        "col": 11,
        "line": 108
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2787,
        "byte_start": 2783,
        "col": 9,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2792,
        "byte_start": 2787,
        "col": 13,
        "line": 110
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2835,
        "byte_start": 2831,
        "col": 11,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2840,
        "byte_start": 2835,
        "col": 15,
        "line": 112
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2887,
        "byte_start": 2883,
        "col": 13,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2892,
        "byte_start": 2887,
        "col": 17,
        "line": 114
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2941,
        "byte_start": 2937,
        "col": 14,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2946,
        "byte_start": 2941,
        "col": 18,
        "line": 116
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2969,
        "byte_start": 2964,
        "col": 18,
        "line": 117
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2992,
        "byte_start": 2987,
        "col": 18,
        "line": 118
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3015,
        "byte_start": 3010,
        "col": 18,
        "line": 119
      }
    },
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 3038,
        "byte_start": 3033,
        "col": 18,
        "line": 120
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1344,
        "byte_start": 1340,
        "col": 5,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 1815,
        "byte_start": 1811,
        "col": 5,
        "line": 62
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2266,
        "byte_start": 2262,
        "col": 5,
        "line": 84
      }
    },
    {
      "category": "Html",
      "code": "html.h1.top_level_heading_warning",
      "message": "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).",
      "severity": "Warning",
      "span": {
        "byte_end": 2703,
        "byte_start": 2699,
        "col": 5,
        "line": 106
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
  "source_name": "html/rendering/non-replaced-elements/sections-and-headings/headings-styles.html"
}
```
