use std::collections::*;
use std::ops::RangeInclusive;

static RULES: [Rule; 20] = [(25..=80, 90..=961),(41..=133, 148..=968),(48..=425, 451..=952),(25..=371, 384..=966),(49..=531, 546..=973),(45..=641, 656..=954),(43..=357, 364..=969),(40..=669, 689..=954),(40..=550, 570..=956),(49..=854, 863..=953),(48..=601, 614..=964),(27..=698, 715..=962),(38..=781, 800..=970),(47..=824, 842..=965),(45..=219, 241..=955),(47..=388, 401..=954),(42..=906, 919..=965),(40..=726, 733..=955),(27..=161, 174..=974),(48..=103, 110..=954)];
static MY_TICKET: Ticket = [181,131,61,67,151,59,113,101,79,53,71,193,179,103,149,157,127,97,73,191];
static TICKETS: [Ticket; 240] = [[522,420,365,200,585,386,904,206,275,61,185,273,550,686,352,338,294,695,503,718],[129,758,494,886,213,178,94,496,423,352,51,863,498,925,653,505,403,314,619,241],[125,766,123,196,205,847,379,480,268,887,546,62,114,209,766,248,501,696,575,76],[183,872,520,282,717,522,776,722,94,717,100,161,208,949,945,255,853,483,117,145],[283,101,290,751,617,334,885,633,767,417,733,887,192,595,62,329,688,187,736,458],[635,466,583,585,733,511,754,106,513,256,347,506,180,403,938,126,886,868,262,481],[895,166,885,272,621,205,739,724,809,777,890,345,303,764,347,253,320,114,750,517],[599,726,584,582,249,347,257,277,320,493,820,147,179,659,271,249,924,615,577,882],[821,422,630,696,938,361,487,895,692,936,160,691,155,332,195,928,357,100,62,122],[770,506,110,123,344,260,114,800,507,889,385,629,884,380,132,328,615,73,245,940],[117,78,800,219,292,527,468,844,872,366,348,410,770,927,314,586,130,621,573,610],[938,123,250,283,724,249,699,939,460,247,334,718,174,483,92,923,417,283,773,195],[698,758,174,364,898,871,74,417,770,203,763,889,510,943,58,425,812,519,63,170],[868,662,900,111,659,455,902,903,803,591,810,173,52,906,215,151,758,503,191,593],[657,161,748,66,331,802,635,411,820,815,746,939,665,923,501,800,53,596,816,680],[585,341,303,495,618,803,420,70,804,550,772,406,255,301,532,282,455,370,76,304],[814,743,411,368,737,868,900,881,418,127,201,407,419,271,321,848,739,24,120,151],[77,338,664,350,181,475,746,639,200,570,619,245,464,908,595,759,663,939,900,151],[639,632,769,8,111,509,155,588,865,324,881,306,587,845,718,205,270,526,265,780],[91,417,286,304,331,628,506,206,492,809,210,747,645,249,809,74,527,577,211,945],[617,739,197,877,774,938,660,747,768,424,695,154,634,324,507,729,503,812,264,895],[50,766,880,571,936,928,80,936,328,178,347,151,922,700,74,617,751,574,628,639],[548,267,850,815,593,627,115,596,764,466,313,68,128,103,263,876,728,282,769,463],[303,906,720,333,348,5,936,191,779,315,216,103,345,411,502,881,311,271,508,62],[895,801,411,804,724,636,118,306,495,632,593,863,729,894,128,270,466,500,510,179],[103,479,250,72,380,824,304,91,184,111,844,62,939,279,209,593,487,492,56,243],[902,617,186,938,483,842,451,213,598,330,241,266,321,854,866,57,695,631,521,545],[745,451,478,634,808,880,784,332,190,586,479,311,472,662,691,813,334,866,264,593],[161,938,725,894,889,285,895,766,78,894,531,780,129,338,987,593,100,813,457,110],[73,317,871,548,760,401,652,352,454,313,418,451,264,71,470,371,323,803,198,899],[417,749,550,177,771,944,813,520,153,760,486,518,488,992,151,753,298,261,280,191],[620,174,771,213,286,121,125,518,616,369,423,243,295,429,268,600,905,266,656,739],[252,93,948,658,473,312,757,314,364,311,882,247,761,500,914,772,690,101,694,875],[458,193,481,309,665,640,491,170,871,470,944,601,198,417,249,620,921,123,666,897],[516,932,506,619,433,726,863,103,762,459,843,194,690,53,573,77,69,158,823,505],[420,773,72,144,928,894,811,333,663,501,694,268,54,413,357,346,868,780,124,261],[488,739,348,635,516,4,269,297,91,201,882,470,460,920,875,418,91,250,277,501],[297,659,824,367,667,457,859,693,198,193,206,292,422,314,514,498,425,127,669,345],[472,466,246,201,755,85,848,804,873,357,800,598,763,592,884,201,496,574,573,264],[92,64,813,65,474,573,505,722,771,188,419,126,867,771,438,720,321,466,588,408],[900,816,866,636,814,317,316,846,9,205,252,54,334,275,181,288,252,691,667,286],[452,253,247,371,469,73,591,51,95,275,50,209,578,725,268,94,192,193,620,383],[948,941,695,939,787,923,869,805,206,267,273,197,820,925,263,101,697,76,219,870],[873,615,477,202,167,202,766,744,823,342,263,94,199,303,804,775,812,174,813,454],[98,319,284,485,209,457,251,866,549,643,183,178,582,866,278,189,592,694,320,943],[218,156,244,468,850,148,344,311,978,116,576,112,849,507,332,752,581,627,405,213],[484,96,517,74,104,253,326,886,628,882,116,526,803,667,770,80,324,306,471,289],[853,61,845,594,302,351,278,819,194,640,306,574,626,247,361,641,504,571,889,801],[405,809,293,243,879,819,215,73,184,695,462,127,198,102,860,813,204,615,245,804],[773,900,854,290,99,773,68,352,150,103,929,614,420,938,585,79,264,676,317,202],[636,666,721,446,67,571,119,97,927,742,262,411,155,591,658,636,747,53,348,201],[478,274,625,58,743,702,206,324,420,762,774,387,890,308,765,597,626,354,867,593],[357,356,290,771,944,758,581,638,701,781,461,131,406,571,482,764,158,187,121,411],[256,463,720,474,753,370,262,261,634,503,474,708,94,421,848,752,341,574,601,818],[202,922,550,670,155,100,275,57,418,769,115,807,768,523,664,579,343,299,407,151],[619,744,477,991,349,179,113,572,583,522,64,938,821,864,410,735,416,879,850,198],[95,728,725,928,588,531,129,578,451,520,722,177,769,206,97,197,660,507,945,127],[344,212,417,890,204,576,298,633,308,524,895,599,246,766,252,638,426,815,519,114],[461,477,130,369,202,520,298,920,505,542,892,203,890,176,487,813,767,640,387,115],[65,73,99,691,773,815,882,243,904,325,719,818,102,369,768,293,407,272,660,9],[204,524,290,761,484,504,351,571,419,778,632,926,577,266,351,735,715,139,808,807],[523,455,721,637,64,848,474,294,716,586,505,274,357,308,456,441,269,724,628,347],[178,666,880,151,94,842,891,591,753,404,885,618,720,86,195,65,281,110,949,851],[121,56,421,856,261,844,638,196,767,847,481,633,751,943,596,385,924,947,810,269],[318,205,415,334,118,571,133,277,323,935,321,278,209,874,184,173,323,176,461,365],[345,735,496,716,54,594,581,736,885,186,897,505,512,159,823,935,13,298,93,472],[724,366,640,207,316,383,632,656,930,493,153,474,905,65,923,765,207,520,339,177],[386,992,925,733,695,287,198,355,66,157,949,775,899,209,200,758,303,94,365,195],[323,186,282,352,638,448,940,181,50,60,473,71,259,308,191,870,293,854,320,739],[90,807,76,998,744,297,315,287,183,281,623,156,275,452,776,657,294,241,589,133],[475,401,508,924,615,206,195,727,616,500,845,131,180,742,577,657,275,118,600,615],[484,890,62,723,477,333,286,668,51,658,62,536,296,807,755,821,931,804,869,639],[351,411,745,355,253,522,944,719,301,813,510,330,69,513,171,101,929,469,347,477],[721,269,206,807,453,897,823,150,200,547,280,70,853,179,839,124,806,586,256,272],[186,942,18,531,587,119,722,906,93,876,353,210,344,496,364,847,150,750,946,802],[630,424,129,808,102,290,742,417,110,580,301,212,804,527,345,94,711,148,633,197],[641,666,640,110,508,351,110,100,345,263,80,77,204,537,751,902,876,691,455,201],[800,939,498,271,122,736,147,615,78,325,300,156,851,242,599,845,946,481,131,717],[118,188,882,349,531,341,200,279,63,776,724,174,7,628,245,574,150,864,148,750],[245,461,210,366,364,461,368,75,669,341,595,127,633,729,619,580,755,938,416,573],[779,191,659,628,259,525,488,735,883,85,185,506,640,371,735,201,354,627,494,846],[616,194,482,589,249,630,278,781,404,470,660,127,403,751,894,740,308,539,739,488],[823,645,507,371,54,733,60,590,271,662,657,508,187,716,301,131,454,272,214,65],[751,522,321,526,85,423,871,656,806,876,190,591,352,211,150,506,816,248,150,522],[821,177,689,490,656,850,390,279,194,127,302,824,412,690,75,186,817,284,661,279],[154,243,126,735,180,926,823,622,692,741,940,778,620,984,328,499,281,779,531,743],[458,276,193,508,206,414,60,770,183,150,893,813,475,121,605,66,485,99,800,885],[621,919,59,65,360,182,717,371,824,919,764,931,341,757,58,639,263,91,205,632],[325,59,161,515,125,508,571,760,351,127,20,317,666,943,804,493,816,177,482,929],[723,476,740,844,701,622,736,806,193,658,119,290,292,54,346,851,756,473,818,51],[693,527,6,932,413,694,103,282,291,809,887,721,665,894,863,306,129,587,338,157],[873,872,474,404,716,289,322,939,474,371,601,199,776,66,919,499,597,879,342,426],[870,570,79,607,411,405,627,757,863,515,892,944,193,339,202,872,926,288,178,596],[79,288,801,942,859,657,801,902,333,186,600,875,497,476,365,878,487,578,344,289],[751,723,919,209,185,586,758,534,310,631,211,810,57,940,158,157,159,254,150,71],[516,823,185,526,75,60,949,295,580,505,254,128,504,104,617,923,694,949,308,334],[758,574,495,868,338,588,342,491,355,601,323,500,497,337,791,802,519,204,745,905],[175,623,271,744,419,306,575,501,720,868,20,202,464,52,197,737,627,199,423,824],[634,100,948,656,244,407,905,178,579,292,800,177,191,315,728,723,621,71,596,889],[621,422,274,770,637,648,601,80,484,53,216,332,659,318,455,98,919,211,694,215],[583,156,348,935,511,773,193,69,112,383,719,94,407,299,179,716,853,354,370,504],[486,761,597,531,763,845,581,97,406,849,526,257,8,516,370,849,410,884,628,265],[723,176,895,457,596,525,366,743,897,371,496,188,320,364,302,57,807,296,986,883],[937,750,75,110,195,904,275,668,805,388,419,468,576,806,476,637,208,115,61,911],[893,672,306,178,898,697,330,407,663,937,296,770,762,364,905,322,204,114,252,250],[732,402,298,210,277,920,343,577,388,598,591,55,343,310,116,153,265,482,212,512],[351,937,889,70,491,269,321,693,594,256,366,331,219,250,196,487,876,632,732,940],[270,301,334,618,948,763,363,317,571,632,474,944,621,50,810,692,843,179,334,472],[822,498,933,599,292,407,873,692,617,404,692,697,882,718,926,770,592,413,980,693],[388,746,726,404,767,268,849,458,13,348,451,364,596,882,188,352,581,497,347,935],[846,521,193,813,158,519,840,386,277,735,253,179,116,599,388,850,812,628,599,940],[814,159,524,241,767,453,472,113,133,150,178,175,591,214,52,648,621,78,406,64],[508,890,843,269,697,508,604,905,666,516,947,929,891,926,283,68,806,406,494,454],[177,590,53,846,79,915,320,313,579,194,75,506,452,190,896,71,69,776,885,201],[756,713,402,103,467,409,421,424,933,921,55,345,418,753,454,148,121,735,206,402],[185,470,762,215,624,212,742,206,734,508,22,52,499,471,348,513,821,801,410,599],[522,874,750,368,588,767,308,486,256,75,510,720,275,794,324,345,178,500,656,71],[123,364,124,276,549,937,194,131,877,175,623,981,251,404,657,181,745,738,779,507],[734,813,388,872,887,461,78,469,411,878,209,445,460,320,847,851,809,261,522,274],[526,181,582,788,216,582,726,619,129,183,254,59,779,58,307,522,253,350,689,101],[985,367,318,290,937,500,509,115,91,310,326,663,251,415,255,881,368,54,284,778],[621,874,520,886,366,493,337,768,863,452,490,297,504,520,683,748,299,588,53,451],[774,854,633,131,780,55,285,419,322,91,618,178,550,596,501,690,698,666,149,783],[693,148,346,527,521,619,718,413,322,305,717,633,900,499,595,9,326,313,320,102],[809,497,771,94,192,494,809,9,601,92,925,638,695,311,258,411,133,629,940,572],[317,293,949,816,740,727,738,755,927,90,285,824,290,188,898,218,810,583,261,206],[500,333,294,501,620,253,212,774,266,156,528,812,178,339,312,884,383,264,638,401],[871,199,734,387,406,77,817,195,124,258,621,571,299,202,622,250,761,712,198,207],[260,313,205,383,218,460,124,812,843,626,921,193,531,495,254,157,927,351,491,388],[213,943,410,666,925,939,370,150,404,443,324,878,55,876,930,887,940,742,306,510],[529,333,892,801,598,294,777,323,200,263,934,219,189,697,288,51,742,996,212,637],[658,417,922,899,891,119,458,121,492,142,263,76,813,91,477,811,56,528,773,823],[805,630,201,465,599,345,513,256,921,267,63,410,631,866,382,498,926,499,465,634],[274,768,414,457,156,516,898,273,579,490,929,744,932,141,251,697,587,130,933,933],[182,410,530,516,366,329,273,280,696,736,277,478,719,62,258,637,620,280,130,862],[271,272,505,148,764,163,512,128,871,112,921,745,112,817,622,291,759,129,195,314],[349,345,901,365,283,621,53,897,67,345,287,985,740,822,898,274,80,583,116,894],[615,503,847,204,309,637,467,943,891,491,879,408,781,77,414,98,331,162,598,760],[321,332,601,244,920,765,523,891,469,471,263,183,119,521,212,854,600,817,9,750],[719,773,473,201,71,341,411,824,479,715,453,766,188,507,582,175,982,264,300,737],[745,899,734,922,526,514,453,433,863,695,364,490,661,766,820,194,187,403,547,460],[455,93,595,53,467,885,213,948,287,744,931,632,134,92,405,634,192,264,313,641],[669,816,895,217,335,149,665,260,842,69,872,328,123,347,115,863,285,316,978,370],[304,813,758,890,666,587,557,208,546,716,259,325,894,201,287,68,258,627,497,723],[69,411,686,252,368,630,319,744,459,482,425,932,624,638,206,261,813,528,691,466],[258,279,169,128,110,263,496,617,264,842,531,852,256,332,625,481,596,331,867,114],[757,669,729,669,341,277,586,122,668,98,349,384,476,767,752,571,573,510,338,665],[112,630,387,904,134,96,494,302,504,750,366,276,512,484,334,120,630,582,809,615],[756,411,470,474,755,573,583,517,599,739,206,589,818,811,564,531,337,854,937,585],[521,68,848,466,312,877,91,467,753,594,201,514,846,575,458,282,173,694,96,901],[736,344,884,844,385,214,906,355,635,900,656,872,324,550,106,470,906,948,60,503],[570,691,699,748,342,462,193,308,290,632,55,716,472,847,865,415,759,503,69,367],[77,279,295,455,726,519,727,740,524,627,734,94,579,589,752,324,904,274,570,772],[571,79,156,764,505,946,150,358,416,920,867,284,939,725,204,121,249,190,265,349],[811,921,924,749,668,534,205,631,548,889,745,280,853,546,948,158,477,578,823,341],[521,631,929,772,495,742,266,710,864,218,770,176,66,94,635,875,767,199,510,949],[578,121,469,875,132,422,614,495,246,593,460,548,745,338,919,709,817,817,334,774],[125,124,546,493,746,91,105,929,769,101,848,800,475,716,521,849,626,243,301,53],[402,341,254,361,76,365,255,345,877,635,741,156,940,755,470,513,472,263,150,883],[595,122,202,107,242,129,755,120,354,767,251,740,334,749,212,820,823,822,504,284],[66,196,876,451,634,288,257,83,498,75,738,848,300,637,721,403,121,364,213,906],[770,314,890,591,865,456,661,250,306,328,211,640,111,74,601,989,531,625,726,903],[768,883,524,578,493,866,987,845,945,595,266,292,479,366,633,724,640,876,697,903],[52,413,423,319,510,503,423,742,481,627,895,197,169,251,193,454,893,174,150,822],[599,425,51,648,364,752,246,204,194,735,937,746,353,500,55,656,518,598,476,592],[637,722,845,634,309,656,818,151,620,251,477,698,530,73,771,721,667,646,275,304],[201,218,473,760,723,178,580,373,512,755,410,348,473,69,906,753,343,847,618,813],[596,582,504,592,949,269,77,809,274,56,308,478,865,358,386,486,62,286,76,323],[347,874,498,355,816,881,738,276,987,386,248,927,516,800,661,583,412,132,262,337],[314,384,474,464,55,881,527,601,749,928,405,695,672,484,631,250,207,272,244,218],[625,119,769,461,280,109,477,297,738,203,342,495,314,150,496,401,889,697,481,180],[79,323,404,470,596,196,473,403,627,454,323,348,689,736,767,749,810,117,660,654],[522,407,814,81,408,735,454,495,182,471,691,126,949,868,597,342,853,408,733,350],[663,980,894,64,62,808,719,112,352,326,69,311,660,303,657,246,901,348,324,530],[947,267,153,893,186,418,415,524,211,102,668,595,774,343,887,94,666,427,187,490],[183,802,205,154,18,926,626,158,771,629,624,270,849,408,589,577,344,332,317,733],[875,95,423,853,588,466,58,570,120,630,599,687,484,53,930,110,96,749,748,657],[257,420,266,501,659,662,641,401,464,884,69,697,269,124,87,808,291,814,160,666],[320,942,186,942,420,76,627,319,289,634,183,132,56,52,990,201,923,266,276,549],[478,310,453,486,496,210,667,983,451,202,523,742,336,581,267,735,56,501,925,313],[487,733,289,334,672,456,570,67,385,402,853,528,501,656,454,809,200,412,760,761],[619,161,582,407,485,487,925,846,128,113,590,600,159,878,925,203,594,728,387,733],[177,492,111,97,541,111,721,414,696,96,187,198,875,636,413,496,90,506,634,622],[661,584,123,187,297,718,263,340,421,297,364,580,735,642,737,477,920,491,297,726],[509,310,781,717,285,182,773,320,595,619,723,583,718,998,295,905,184,188,118,760],[277,247,897,120,365,219,252,269,464,54,291,628,270,887,926,315,0,318,196,753],[325,664,63,526,283,632,761,326,185,54,252,584,810,60,501,180,775,505,201,104],[203,585,76,889,351,53,672,123,750,884,452,871,262,526,344,368,864,252,483,174],[204,496,202,186,276,850,130,148,730,60,722,364,852,823,131,692,185,519,528,927],[473,845,807,904,330,944,172,294,425,203,760,494,754,371,371,756,385,158,203,815],[886,261,804,457,876,680,903,323,386,79,357,505,689,192,889,546,293,462,458,697],[490,122,336,628,462,717,271,749,128,873,947,219,51,180,368,928,647,622,809,758],[209,301,693,268,402,205,850,511,156,641,251,343,328,757,850,179,669,592,125,727],[765,581,663,218,634,689,663,209,898,527,151,86,508,852,843,571,630,345,824,284],[156,660,219,880,516,529,487,128,284,630,211,729,314,589,621,886,114,254,502,98],[756,634,149,120,739,975,131,479,300,515,615,129,192,762,130,767,474,371,216,175],[129,251,219,853,214,580,210,137,325,770,305,669,627,58,496,295,324,524,311,112],[490,466,479,664,593,176,808,587,876,731,866,623,350,333,298,284,585,932,258,657],[206,488,461,162,457,124,212,853,802,743,126,342,583,313,576,339,870,478,940,80],[906,640,942,330,90,159,317,424,329,801,482,153,697,525,891,697,73,724,191,359],[401,313,878,294,384,531,917,506,156,369,622,766,54,329,280,850,71,341,938,668],[326,495,321,669,756,882,775,664,849,932,387,354,823,515,817,132,15,776,64,942],[474,404,659,250,496,614,631,528,639,820,847,259,930,93,661,136,581,582,689,884],[97,413,460,570,151,113,942,465,576,846,574,301,101,300,133,255,744,347,477,706],[423,530,500,251,600,504,899,767,751,735,425,407,747,292,135,322,843,424,819,757],[286,499,269,402,495,405,451,690,69,92,156,261,709,110,349,616,550,347,276,667],[157,717,150,880,805,484,178,615,949,872,622,653,59,870,473,657,694,696,818,298],[663,341,366,811,71,627,211,459,633,578,501,920,658,893,249,525,996,812,76,888],[413,863,278,717,728,369,800,626,871,402,842,75,175,501,724,641,774,206,756,882],[129,112,343,574,667,484,889,681,875,588,938,285,470,337,885,160,849,309,310,549],[760,300,802,284,624,847,266,760,499,95,98,106,212,480,667,260,409,325,744,503],[344,851,928,694,482,218,439,263,890,810,482,341,771,570,949,938,773,268,885,547],[246,512,591,190,740,357,545,904,851,65,581,507,574,803,336,326,346,803,249,475],[588,816,178,885,253,757,334,158,686,810,527,200,314,451,97,126,493,583,668,935],[355,589,498,126,182,264,734,304,890,487,919,999,297,346,928,876,531,948,633,420],[818,930,402,526,854,271,408,948,748,473,759,297,722,715,290,56,627,382,330,419],[128,822,619,196,869,802,697,183,405,384,202,470,209,341,712,57,322,346,846,285],[97,844,819,150,495,270,904,508,218,573,458,120,823,167,658,406,938,772,772,208],[578,350,932,936,268,464,807,258,598,706,766,66,250,265,878,319,741,388,365,574],[600,478,897,475,473,899,586,822,738,720,202,598,156,522,325,683,769,254,124,112],[519,843,850,384,177,146,157,122,64,124,871,514,895,779,157,808,346,286,244,124],[498,775,75,852,571,249,467,249,692,816,621,620,276,184,423,369,848,547,747,85],[124,938,457,773,644,854,256,822,60,480,317,76,504,751,751,530,781,285,764,323],[801,781,110,354,918,411,633,752,583,335,668,504,803,118,482,199,58,498,756,456],[294,820,600,545,257,176,186,262,482,125,893,241,662,747,352,853,324,250,592,253],[698,307,745,303,411,249,356,154,737,282,736,374,124,751,667,808,665,195,625,299],[696,60,778,736,617,817,871,981,578,384,202,270,70,295,351,852,346,248,318,339],[204,499,635,287,125,308,75,863,189,675,155,417,353,547,319,219,769,111,468,292],[249,624,59,701,283,256,280,280,293,217,414,407,419,262,127,516,872,866,667,620],[570,735,944,728,615,467,546,122,763,356,50,214,515,265,530,748,596,256,66,635],[98,161,525,69,583,663,112,653,599,423,527,817,741,778,454,664,284,664,872,525],[803,781,760,259,56,520,666,244,724,821,719,9,721,719,813,618,816,744,941,248],[869,923,312,388,937,213,86,319,770,743,693,847,283,469,806,419,947,186,279,513],[453,340,342,746,245,284,339,847,214,597,692,845,598,244,452,114,139,854,752,62],[757,186,207,250,510,151,928,256,419,757,690,321,824,127,306,823,199,630,874,828],[270,127,339,311,70,349,513,458,286,996,305,721,152,473,570,161,460,299,356,261],[411,286,852,916,763,854,412,312,842,113,345,258,289,346,869,256,453,929,468,161],[462,120,847,638,868,316,583,404,403,168,193,175,213,721,184,349,364,217,775,880],[933,266,195,584,902,322,338,930,987,494,306,309,182,150,421,870,307,100,761,304],[744,515,822,903,633,639,883,257,945,943,666,156,353,878,155,198,518,930,707,753]];

type Rule = (RangeInclusive<usize>, RangeInclusive<usize>);
type Ticket = [usize; 20];

fn fits_rule((r1,r2): &Rule, v: usize) -> bool {
  r1.contains(&v) || r2.contains(&v)
}

fn part_one() -> (usize, Vec<&'static Ticket>) {
  let mut ans = 0;
  let mut tickets = Vec::new();
  tickets.push(&MY_TICKET);
  for t in &TICKETS {
    match t.iter().find(|&&v| RULES.iter().all(|r| !fits_rule(r,v))) {
      Some(v) => ans += v,
      None    => tickets.push(t),
    }
  }
  (ans, tickets)
}

fn get_possible_rules(tickets: &[&Ticket]) -> Vec<HashSet<usize>> {
  (0..20).map(|i|
    (0..20).filter(|&j|
      tickets.iter().all(|t| fits_rule(&RULES[j], t[i]))
    ).collect()
  ).collect()
}

fn part_two(tickets: &[&Ticket]) -> usize {
  let mut possible_rules = get_possible_rules(&tickets);
  let mut assigned_rules = [0; 20];
  loop {
    let (i,&v) = match possible_rules.iter().enumerate().find(|(_,s)| s.len() == 1) {
      Some((i,s)) => (i, s.iter().next().unwrap()),
      None => break,
    };
    assigned_rules[i] = v;
    for s in &mut possible_rules { s.remove(&v); }
  }

  assigned_rules.iter()
    .enumerate()
    .filter(|(_,&rule)| rule < 6)
    .map(|(i,_)| MY_TICKET[i])
    .product()
}

aoc2020::main! {
  let (p1, tickets) = part_one();
  let p2 = part_two(&tickets);
  (p1, p2)
}
