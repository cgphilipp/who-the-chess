# Wikidata API

- https://query.wikidata.org/querybuilder/
- https://query.wikidata.org/

## Getting all 2700 players

```
SELECT DISTINCT ?item ?itemLabel  WHERE {
  SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE]". }
  {
    SELECT DISTINCT ?item WHERE {
      ?item wdt:P31 wd:Q5 .   # human
      ?item p:P1087 ?statement0.
      ?statement0 (psv:P1087/wikibase:quantityAmount) ?numericQuantity.
      FILTER(?numericQuantity > "2700"^^xsd:decimal)
    }
    LIMIT 350
  }
}

```

## Source code for the dump

Only later I found that there may be a simpler approach: https://stackoverflow.com/questions/66918787/get-all-properties-sub-properties-and-label-ids-from-wikidata-item

```
SELECT ?playerLabel ?wdLabel ?ps_Label ?wdpqLabel ?pq_Label {
  VALUES (?player) {
(wd:Q45747)
(wd:Q445661)
(wd:Q964913)
(wd:Q709548)
(wd:Q993350)
(wd:Q460020)
(wd:Q1364609)
(wd:Q337223)
(wd:Q708938)
(wd:Q15877133)
(wd:Q2064538)
(wd:Q486778)
(wd:Q13646741)
(wd:Q253524)
(wd:Q276885)
(wd:Q278777)
(wd:Q361454)
(wd:Q367277)
(wd:Q489015)
(wd:Q548946)
(wd:Q556485)
(wd:Q102648)
(wd:Q312918)
(wd:Q315155)
(wd:Q363938)
(wd:Q371180)
(wd:Q377866)
(wd:Q378734)
(wd:Q380839)
(wd:Q465524)
(wd:Q470788)
(wd:Q506421)
(wd:Q525731)
(wd:Q541729)
(wd:Q555598)
(wd:Q703625)
(wd:Q726187)
(wd:Q727947)
(wd:Q955884)
(wd:Q1383467)
(wd:Q1450826)
(wd:Q15068870)
(wd:Q15614436)
(wd:Q16336094)
(wd:Q115814)
(wd:Q102664)
(wd:Q106807)
(wd:Q20908925)
(wd:Q208229)
(wd:Q256923)
(wd:Q392661)
(wd:Q437865)
(wd:Q574962)
(wd:Q667067)
(wd:Q709256)
(wd:Q2004661)
(wd:Q3078552)
(wd:Q147495)
(wd:Q20973724)
(wd:Q312267)
(wd:Q535460)
(wd:Q543256)
(wd:Q550688)
(wd:Q670159)
(wd:Q734168)
(wd:Q923453)
(wd:Q932307)
(wd:Q937577)
(wd:Q961090)
(wd:Q22338182)
(wd:Q4225422)
(wd:Q27524878)
(wd:Q27526478)
(wd:Q59137809)
(wd:Q218875)
(wd:Q7412591)
(wd:Q77224)
(wd:Q93674)
(wd:Q319380)
(wd:Q352413)
(wd:Q504960)
(wd:Q938831)
(wd:Q1335244)
(wd:Q26703041)
(wd:Q161092)
(wd:Q217198)
(wd:Q442544)
(wd:Q543241)
(wd:Q795182)
(wd:Q956390)
(wd:Q968844)
(wd:Q1356780)
(wd:Q27524847)
(wd:Q27527529)
(wd:Q10860652)
(wd:Q562700)
(wd:Q18923582)
(wd:Q41314)
(wd:Q27524449)
(wd:Q28614)
(wd:Q106374)
(wd:Q154586)
(wd:Q172798)
(wd:Q183250)
(wd:Q185236)
(wd:Q210922)
(wd:Q214713)
(wd:Q313778)
(wd:Q521817)
(wd:Q688382)
(wd:Q793526)
(wd:Q131674)
(wd:Q299636)
(wd:Q893748)
(wd:Q1119406)
(wd:Q1152824)
(wd:Q1191198)
(wd:Q1713972)
(wd:Q4482094)
(wd:Q289349)
(wd:Q490636)
(wd:Q555232)
(wd:Q591200)
(wd:Q718592)
(wd:Q728179)
(wd:Q945565)
(wd:Q1232768)
(wd:Q2218873)
(wd:Q15955505)
(wd:Q207723)
(wd:Q211634)
(wd:Q251611)
(wd:Q314531)
(wd:Q60779)
(wd:Q56026072)
}
  
  ?player ?p ?statement .
  ?statement ?ps ?ps_ .
  
  ?wd wikibase:claim ?p.
  ?wd wikibase:statementProperty ?ps.
  
  OPTIONAL {
  ?statement ?pq ?pq_ .
  ?wdpq wikibase:qualifier ?pq .
  }
  
  SERVICE wikibase:label { bd:serviceParam wikibase:language "en" }
}

```
