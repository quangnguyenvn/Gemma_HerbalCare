import { writable } from 'svelte/store';

export type Locale = 'en' | 'sw' | 'hi' | 'zh' | 'ko';

export const languageOptions: Array<{ code: Locale; label: string }> = [
  { code: 'en', label: 'English' },
  { code: 'sw', label: 'Swahili' },
  { code: 'hi', label: 'हिन्दी' },
  { code: 'zh', label: '中文' },
  { code: 'ko', label: '한국어' }
];

export const navCopy: Record<
  Locale,
  {
    navLabel: string;
    home: string;
    consult: string;
    herbs: string;
    safety: string;
    language: string;
  }
> = {
  en: {
    navLabel: 'Primary navigation',
    home: 'Home',
    consult: 'Consult',
    herbs: 'Herb library',
    safety: 'Safety',
    language: 'Language'
  },
  sw: {
    navLabel: 'Urambazaji mkuu',
    home: 'Nyumbani',
    consult: 'Ushauri',
    herbs: 'Maktaba ya mimea',
    safety: 'Usalama',
    language: 'Lugha'
  },
  hi: {
    navLabel: 'मुख्य नेविगेशन',
    home: 'होम',
    consult: 'सलाह',
    herbs: 'जड़ी-बूटी सूची',
    safety: 'सुरक्षा',
    language: 'भाषा'
  },
  zh: {
    navLabel: '主导航',
    home: '首页',
    consult: '咨询',
    herbs: '草药库',
    safety: '安全',
    language: '语言'
  },
  ko: {
    navLabel: '기본 탐색',
    home: '홈',
    consult: '상담',
    herbs: '허브 라이브러리',
    safety: '안전',
    language: '언어'
  }
};

export const homeCopy: Record<
  Locale,
  {
    eyebrow: string;
    subtitle: string;
    start: string;
    safetyPolicy: string;
    triageTitle: string;
    triageBody: string;
    recordsTitle: string;
    recordsBody: string;
    gemmaTitle: string;
    gemmaBody: string;
    mobileEyebrow: string;
    mobileTitle: string;
    mobileBody: string;
    phoneTitle: string;
    phoneMoringaPack: string;
    phoneGingerPack: string;
    phoneOffline: string;
    offlineTitle: string;
    offlineBody: string;
    updatesTitle: string;
    updatesBody: string;
    fieldTitle: string;
    fieldBody: string;
  }
> = {
  en: {
    eyebrow: 'Safety-first herbal knowledge navigator',
    subtitle:
      'Safety-first local herbal guidance for low-resource communities, with red-flag triage before any herb is shown.',
    start: 'Start consultation',
    safetyPolicy: 'Read safety policy',
    triageTitle: 'Triage before tradition',
    triageBody:
      'Emergency symptoms return urgent-care guidance only. Herbs are hidden when chest pain, difficulty breathing, severe dehydration, cancer cure claims, and other red flags appear.',
    recordsTitle: 'Local, curated records',
    recordsBody:
      'The demo uses a small SQLite dataset with source URLs, evidence levels, safety summaries, contraindications, interactions, and regional availability.',
    gemmaTitle: 'Gemma-ready core',
    gemmaBody:
      'A mock provider ships first. Switch to an HTTP Gemma endpoint with environment variables when the inference target is ready.',
    mobileEyebrow: 'Mobile and offline direction',
    mobileTitle: 'Herb knowledge on iOS and Android',
    mobileBody:
      'Phase 2 can package the Herb Library as a phone app for iOS and Android, helping families and community workers browse local plants, likely uses, identification notes, safety warnings, and regional availability even when connectivity is weak.',
    phoneTitle: 'Regional Herb Library',
    phoneMoringaPack: 'Kano pack',
    phoneGingerPack: 'Bihar pack',
    phoneOffline: 'Offline ready',
    offlineTitle: 'Offline-first library:',
    offlineBody: 'cache trusted regional herb and food-plant records on the device.',
    updatesTitle: 'Local updates:',
    updatesBody: 'sync new reviewed records when the phone reconnects.',
    fieldTitle: 'Field use:',
    fieldBody: 'keep safety triage, source links, and visual guides close to the people who need them.'
  },
  sw: {
    eyebrow: 'Mwongozo wa mimea unaotanguliza usalama',
    subtitle:
      'Mwongozo wa mimea ya eneo kwa jamii zenye rasilimali chache, huku dalili hatari zikikaguliwa kabla mimea haijaonyeshwa.',
    start: 'Anza ushauri',
    safetyPolicy: 'Soma sera ya usalama',
    triageTitle: 'Kagua hatari kabla ya tiba za jadi',
    triageBody:
      'Dalili za dharura hutoa mwongozo wa kupata huduma haraka tu. Mimea hufichwa ikiwa kuna maumivu ya kifua, kupumua kwa shida, upungufu mkubwa wa maji, madai ya kutibu kansa, au dalili nyingine hatari.',
    recordsTitle: 'Rekodi za eneo zilizochaguliwa',
    recordsBody:
      'Demo hutumia dataset ndogo ya SQLite yenye viungo vya vyanzo, kiwango cha ushahidi, muhtasari wa usalama, tahadhari, mwingiliano wa dawa, na upatikanaji wa eneo.',
    gemmaTitle: 'Msingi ulio tayari kwa Gemma',
    gemmaBody:
      'Kwanza kuna mock provider. Badilisha kwenda HTTP Gemma endpoint kwa environment variables wakati inference target iko tayari.',
    mobileEyebrow: 'Mwelekeo wa simu na offline',
    mobileTitle: 'Maarifa ya mimea kwenye iOS na Android',
    mobileBody:
      'Phase 2 inaweza kufunga Herb Library kama app ya simu kwa iOS na Android, ikisaidia familia na wahudumu wa jamii kuona mimea ya eneo, matumizi yanayowezekana, njia za kuitambua, tahadhari za usalama, na upatikanaji wa eneo hata mtandao ukiwa dhaifu.',
    phoneTitle: 'Maktaba ya Mimea ya Eneo',
    phoneMoringaPack: 'Kifurushi cha Kano',
    phoneGingerPack: 'Kifurushi cha Bihar',
    phoneOffline: 'Tayari offline',
    offlineTitle: 'Maktaba ya offline kwanza:',
    offlineBody: 'hifadhi rekodi za mimea na chakula cha eneo kwenye kifaa.',
    updatesTitle: 'Sasisho za eneo:',
    updatesBody: 'linganisha rekodi mpya zilizokaguliwa simu ikiunganishwa tena.',
    fieldTitle: 'Matumizi ya field:',
    fieldBody: 'weka triage ya usalama, viungo vya vyanzo, na picha karibu na watu wanaozihitaji.'
  },
  hi: {
    eyebrow: 'सुरक्षा-पहले जड़ी-बूटी ज्ञान मार्गदर्शक',
    subtitle:
      'कम संसाधन वाले समुदायों के लिए स्थानीय जड़ी-बूटी मार्गदर्शन, जिसमें कोई भी जड़ी-बूटी दिखाने से पहले खतरे के संकेत जांचे जाते हैं।',
    start: 'परामर्श शुरू करें',
    safetyPolicy: 'सुरक्षा नीति पढ़ें',
    triageTitle: 'परंपरा से पहले सुरक्षा जांच',
    triageBody:
      'आपातकालीन लक्षणों में केवल तुरंत देखभाल की सलाह दी जाती है। सीने में दर्द, सांस लेने में कठिनाई, गंभीर निर्जलीकरण, कैंसर इलाज के दावे, या अन्य खतरे दिखने पर जड़ी-बूटी छिपा दी जाती है।',
    recordsTitle: 'स्थानीय, चुने हुए रिकॉर्ड',
    recordsBody:
      'डेमो में एक छोटा SQLite dataset है जिसमें source URLs, evidence levels, safety summaries, contraindications, interactions, और regional availability शामिल हैं।',
    gemmaTitle: 'Gemma-ready core',
    gemmaBody:
      'पहले mock provider आता है। inference target तैयार होने पर environment variables से HTTP Gemma endpoint पर बदला जा सकता है।',
    mobileEyebrow: 'मोबाइल और offline दिशा',
    mobileTitle: 'iOS और Android पर जड़ी-बूटी ज्ञान',
    mobileBody:
      'Phase 2 में Herb Library को iOS और Android phone app के रूप में पैक किया जा सकता है, ताकि परिवार और community workers कमजोर internet में भी स्थानीय पौधे, उपयोग, पहचान नोट, safety warnings, और regional availability देख सकें।',
    phoneTitle: 'Regional Herb Library',
    phoneMoringaPack: 'Kano pack',
    phoneGingerPack: 'Bihar pack',
    phoneOffline: 'Offline ready',
    offlineTitle: 'Offline-first library:',
    offlineBody: 'विश्वसनीय regional herb और food-plant records को device पर cache करें।',
    updatesTitle: 'Local updates:',
    updatesBody: 'phone reconnect होने पर नए reviewed records sync करें।',
    fieldTitle: 'Field use:',
    fieldBody: 'safety triage, source links, और visual guides को जरूरतमंद लोगों के पास रखें।'
  },
  zh: {
    eyebrow: '安全优先的草药知识导航',
    subtitle:
      '为资源有限社区提供本地草药指导；在显示任何草药之前，先检查危险信号。',
    start: '开始咨询',
    safetyPolicy: '阅读安全政策',
    triageTitle: '传统知识之前先分诊',
    triageBody:
      '出现紧急症状时，只显示紧急就医指导。若有胸痛、呼吸困难、严重脱水、癌症治疗请求或其他危险信号，草药建议会被隐藏。',
    recordsTitle: '本地精选记录',
    recordsBody:
      '演示使用小型 SQLite 数据集，包含来源链接、证据等级、安全摘要、禁忌、相互作用和地区可用性。',
    gemmaTitle: 'Gemma-ready core',
    gemmaBody:
      '系统默认提供 mock provider。推理目标准备好后，可通过环境变量切换到 HTTP Gemma endpoint。',
    mobileEyebrow: '移动端和离线方向',
    mobileTitle: 'iOS 和 Android 上的草药知识',
    mobileBody:
      'Phase 2 可将 Herb Library 打包为 iOS 和 Android 手机应用，帮助家庭和社区工作者在网络较弱时浏览本地植物、可能用途、识别说明、安全警告和地区可用性。',
    phoneTitle: '区域草药库',
    phoneMoringaPack: 'Kano pack',
    phoneGingerPack: 'Bihar pack',
    phoneOffline: '可离线使用',
    offlineTitle: '离线优先库：',
    offlineBody: '在设备上缓存可信的区域草药和食用植物记录。',
    updatesTitle: '本地更新：',
    updatesBody: '手机重新联网后同步新的已审核记录。',
    fieldTitle: '现场使用：',
    fieldBody: '把安全分诊、来源链接和视觉指南带到真正需要的人身边。'
  },
  ko: {
    eyebrow: '안전 우선 허브 지식 내비게이터',
    subtitle:
      '자원이 부족한 지역을 위한 현지 허브 안내입니다. 어떤 허브를 보여주기 전에 위험 신호를 먼저 확인합니다.',
    start: '상담 시작',
    safetyPolicy: '안전 정책 보기',
    triageTitle: '전통 지식보다 먼저 안전 확인',
    triageBody:
      '응급 증상이 있으면 긴급 진료 안내만 보여줍니다. 흉통, 호흡 곤란, 심한 탈수, 암 치료 주장, 기타 위험 신호가 있으면 허브 추천은 숨겨집니다.',
    recordsTitle: '현지 검토 기록',
    recordsBody:
      '데모는 작은 SQLite 데이터셋을 사용하며 출처 URL, 근거 수준, 안전 요약, 금기, 상호작용, 지역별 이용 가능성을 포함합니다.',
    gemmaTitle: 'Gemma-ready core',
    gemmaBody:
      '먼저 mock provider가 포함되어 있습니다. 추론 대상이 준비되면 환경 변수로 HTTP Gemma endpoint로 전환할 수 있습니다.',
    mobileEyebrow: '모바일 및 오프라인 방향',
    mobileTitle: 'iOS와 Android의 허브 지식',
    mobileBody:
      'Phase 2에서는 Herb Library를 iOS와 Android 앱으로 패키징하여, 인터넷이 약한 환경에서도 가족과 커뮤니티 워커가 현지 식물, 가능한 용도, 식별 노트, 안전 경고, 지역별 이용 가능성을 볼 수 있게 합니다.',
    phoneTitle: '지역 허브 라이브러리',
    phoneMoringaPack: 'Kano pack',
    phoneGingerPack: 'Bihar pack',
    phoneOffline: '오프라인 준비',
    offlineTitle: '오프라인 우선 라이브러리:',
    offlineBody: '신뢰할 수 있는 지역 허브와 식용 식물 기록을 기기에 캐시합니다.',
    updatesTitle: '현지 업데이트:',
    updatesBody: '휴대폰이 다시 연결되면 검토된 새 기록을 동기화합니다.',
    fieldTitle: '현장 사용:',
    fieldBody: '안전 분류, 출처 링크, 시각 가이드를 필요한 사람 가까이에 둡니다.'
  }
};

export const locale = writable<Locale>('en');

export function isLocale(value: string | null): value is Locale {
  return value === 'en' || value === 'sw' || value === 'hi' || value === 'zh' || value === 'ko';
}
