import { writable } from 'svelte/store';

export type Locale =
  | 'en'
  | 'sw'
  | 'hi'
  | 'zh'
  | 'ko'
  | 'vi-VN'
  | 'lo-LA'
  | 'my-MM'
  | 'es-PE'
  | 'es-BO'
  | 'fa-AF'
  | 'rn-BI';

export const languageOptions: Array<{ code: Locale; label: string }> = [
  { code: 'en', label: 'English' },
  { code: 'sw', label: 'Swahili' },
  { code: 'hi', label: 'हिन्दी' },
  { code: 'zh', label: '中文' },
  { code: 'ko', label: '한국어' },
  { code: 'vi-VN', label: 'Tiếng Việt' },
  { code: 'lo-LA', label: 'ລາວ' },
  { code: 'my-MM', label: 'မြန်မာ' },
  { code: 'es-PE', label: 'Español (Perú)' },
  { code: 'es-BO', label: 'Español (Bolivia)' },
  { code: 'fa-AF', label: 'دری (Afghanistan)' },
  { code: 'rn-BI', label: 'Kirundi (Burundi)' }
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
  },
  'vi-VN': {
    navLabel: 'Điều hướng chính',
    home: 'Trang chủ',
    consult: 'Tư vấn',
    herbs: 'Thư viện thảo mộc',
    safety: 'An toàn',
    language: 'Ngôn ngữ'
  },
  'lo-LA': {
    navLabel: 'ການນຳທາງຫຼັກ',
    home: 'ໜ້າຫຼັກ',
    consult: 'ປຶກສາ',
    herbs: 'ຫ້ອງສະໝຸດສະໝຸນໄພ',
    safety: 'ຄວາມປອດໄພ',
    language: 'ພາສາ'
  },
  'my-MM': {
    navLabel: 'အဓိက လမ်းညွှန်',
    home: 'ပင်မ',
    consult: 'တိုင်ပင်ရန်',
    herbs: 'ဆေးဖက်ဝင်အပင် စာကြည့်တိုက်',
    safety: 'ဘေးကင်းရေး',
    language: 'ဘာသာစကား'
  },
  'es-PE': {
    navLabel: 'Navegación principal',
    home: 'Inicio',
    consult: 'Consulta',
    herbs: 'Biblioteca de plantas',
    safety: 'Seguridad',
    language: 'Idioma'
  },
  'es-BO': {
    navLabel: 'Navegación principal',
    home: 'Inicio',
    consult: 'Consulta',
    herbs: 'Biblioteca de plantas',
    safety: 'Seguridad',
    language: 'Idioma'
  },
  'fa-AF': {
    navLabel: 'ناوبری اصلی',
    home: 'خانه',
    consult: 'مشوره',
    herbs: 'کتابخانه گیاهان',
    safety: 'ایمنی',
    language: 'زبان'
  },
  'rn-BI': {
    navLabel: 'Inzira nyamukuru',
    home: 'Ahabanza',
    consult: 'Inama',
    herbs: "Isomero ry'ibimera",
    safety: 'Umutekano',
    language: 'Ururimi'
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
  },
  'vi-VN': {
    eyebrow: 'Trình dẫn kiến thức thảo mộc ưu tiên an toàn',
    subtitle:
      'Hướng dẫn thảo mộc địa phương cho cộng đồng ít nguồn lực, luôn kiểm tra dấu hiệu nguy hiểm trước khi hiển thị bất kỳ cây thuốc nào.',
    start: 'Bắt đầu tư vấn',
    safetyPolicy: 'Đọc chính sách an toàn',
    triageTitle: 'Sàng lọc an toàn trước tri thức truyền thống',
    triageBody:
      'Triệu chứng khẩn cấp chỉ trả về hướng dẫn tìm chăm sóc y tế. Thảo mộc sẽ bị ẩn khi có đau ngực, khó thở, mất nước nặng, yêu cầu chữa ung thư hoặc các dấu hiệu nguy hiểm khác.',
    recordsTitle: 'Bản ghi địa phương đã chọn lọc',
    recordsBody:
      'Bản demo dùng một tập dữ liệu SQLite nhỏ có URL nguồn, mức bằng chứng, tóm tắt an toàn, chống chỉ định, tương tác và mức sẵn có theo vùng.',
    gemmaTitle: 'Lõi sẵn sàng cho Gemma',
    gemmaBody:
      'Mặc định có mock provider để demo. Khi mục tiêu suy luận sẵn sàng, có thể chuyển sang HTTP Gemma endpoint bằng biến môi trường.',
    mobileEyebrow: 'Hướng mobile và offline',
    mobileTitle: 'Kiến thức thảo mộc trên iOS và Android',
    mobileBody:
      'Ở Phase 2, Herb Library có thể đóng gói thành app iOS và Android, giúp gia đình và nhân viên cộng đồng xem cây địa phương, công dụng có thể có, ghi chú nhận dạng, cảnh báo an toàn và mức sẵn có theo vùng ngay cả khi kết nối yếu.',
    phoneTitle: 'Thư viện thảo mộc theo vùng',
    phoneMoringaPack: 'Gói Kano',
    phoneGingerPack: 'Gói Bihar',
    phoneOffline: 'Sẵn sàng offline',
    offlineTitle: 'Thư viện ưu tiên offline:',
    offlineBody: 'lưu trên thiết bị các bản ghi thảo mộc và cây thực phẩm theo vùng đã được tin cậy.',
    updatesTitle: 'Cập nhật địa phương:',
    updatesBody: 'đồng bộ bản ghi đã duyệt khi điện thoại có kết nối trở lại.',
    fieldTitle: 'Dùng tại thực địa:',
    fieldBody: 'giữ sàng lọc an toàn, liên kết nguồn và hướng dẫn trực quan ở gần người cần chúng.'
  },
  'lo-LA': {
    eyebrow: 'ຕົວນຳທາງຄວາມຮູ້ສະໝຸນໄພທີ່ໃຫ້ຄວາມປອດໄພກ່ອນ',
    subtitle:
      'ຄຳແນະນຳສະໝຸນໄພທ້ອງຖິ່ນສຳລັບຊຸມຊົນທີ່ມີຊັບພະຍາກອນໜ້ອຍ, ກວດສັນຍານອັນຕະລາຍກ່ອນສະແດງພືດສະໝຸນໄພ.',
    start: 'ເລີ່ມປຶກສາ',
    safetyPolicy: 'ອ່ານນະໂຍບາຍຄວາມປອດໄພ',
    triageTitle: 'ກວດຄວາມປອດໄພກ່ອນຄວາມຮູ້ດັ້ງເດີມ',
    triageBody:
      'ອາການສຸກເສີນຈະໄດ້ຮັບຄຳແນະນຳໃຫ້ໄປຮັບການດູແລດ່ວນເທົ່ານັ້ນ. ຄຳແນະນຳສະໝຸນໄພຈະຖືກເຊື່ອງເມື່ອມີເຈັບໜ້າເອິກ, ຫາຍໃຈຍາກ, ຂາດນ້ຳຮຸນແຮງ, ຄຳຂໍຮັກສາມະເຮັງ ຫຼື ສັນຍານອັນຕະລາຍອື່ນ.',
    recordsTitle: 'ບັນທຶກທ້ອງຖິ່ນທີ່ຄັດເລືອກແລ້ວ',
    recordsBody:
      'ຕົວຢ່າງນີ້ໃຊ້ຊຸດຂໍ້ມູນ SQLite ຂະໜາດນ້ອຍ ທີ່ມີ URL ແຫຼ່ງຂໍ້ມູນ, ລະດັບຫຼັກຖານ, ສະຫຼຸບຄວາມປອດໄພ, ຂໍ້ຫ້າມ, ປະຕິສຳພັນ ແລະ ຄວາມພ້ອມໃຊ້ຕາມພື້ນທີ່.',
    gemmaTitle: 'ແກນກາງທີ່ພ້ອມສຳລັບ Gemma',
    gemmaBody:
      'ຕົວຢ່າງເລີ່ມດ້ວຍ mock provider. ເມື່ອ inference target ພ້ອມ ສາມາດປ່ຽນໄປຫາ HTTP Gemma endpoint ດ້ວຍ environment variables.',
    mobileEyebrow: 'ທິດທາງມືຖື ແລະ offline',
    mobileTitle: 'ຄວາມຮູ້ສະໝຸນໄພໃນ iOS ແລະ Android',
    mobileBody:
      'ໃນ Phase 2, Herb Library ສາມາດຖືກຈັດເປັນແອັບ iOS ແລະ Android ເພື່ອຊ່ວຍຄອບຄົວ ແລະ ອາສາສະໝັກຊຸມຊົນຄົ້ນຫາພືດທ້ອງຖິ່ນ, ການນຳໃຊ້ທີ່ເປັນໄປໄດ້, ວິທີຈຳແນກ, ຄຳເຕືອນຄວາມປອດໄພ ແລະ ຄວາມພ້ອມໃຊ້ໃນພື້ນທີ່ແມ່ນແມ້ເຄືອຂ່າຍອ່ອນ.',
    phoneTitle: 'ຫ້ອງສະໝຸດສະໝຸນໄພຕາມພື້ນທີ່',
    phoneMoringaPack: 'ຊຸດ Kano',
    phoneGingerPack: 'ຊຸດ Bihar',
    phoneOffline: 'ພ້ອມໃຊ້ offline',
    offlineTitle: 'ຫ້ອງສະໝຸດ offline-first:',
    offlineBody: 'ເກັບບັນທຶກສະໝຸນໄພ ແລະ ພືດອາຫານທ້ອງຖິ່ນທີ່ເຊື່ອຖືໄດ້ໄວ້ໃນອຸປະກອນ.',
    updatesTitle: 'ອັບເດດທ້ອງຖິ່ນ:',
    updatesBody: 'sync ບັນທຶກໃໝ່ທີ່ກວດແລ້ວເມື່ອໂທລະສັບກັບມາເຊື່ອມຕໍ່.',
    fieldTitle: 'ໃຊ້ໃນພາກສະໜາມ:',
    fieldBody: 'ເກັບການກວດຄວາມປອດໄພ, ລິ້ງແຫຼ່ງຂໍ້ມູນ ແລະ ຄູ່ມືຮູບພາບໃຫ້ໃກ້ກັບຄົນທີ່ຕ້ອງການ.'
  },
  'my-MM': {
    eyebrow: 'ဘေးကင်းရေးကို ဦးစားပေးသော ဆေးဖက်ဝင်အပင် အသိပညာလမ်းညွှန်',
    subtitle:
      'အရင်းအမြစ်နည်းသော လူမှုအသိုင်းအဝိုင်းများအတွက် ဒေသတွင်းဆေးဖက်ဝင်အပင် လမ်းညွှန်ဖြစ်ပြီး အပင်မပြမီ အန္တရာယ်လက္ခဏာများကို အရင်စစ်ဆေးသည်။',
    start: 'တိုင်ပင်မှု စတင်ရန်',
    safetyPolicy: 'ဘေးကင်းရေးမူဝါဒ ဖတ်ရန်',
    triageTitle: 'ရိုးရာအသိပညာမတိုင်မီ ဘေးကင်းရေးစစ်ဆေးခြင်း',
    triageBody:
      'အရေးပေါ်လက္ခဏာများရှိပါက အရေးပေါ်ဆေးကုသမှုရှာရန် လမ်းညွှန်ချက်သာ ပြသသည်။ ရင်ဘတ်နာခြင်း, အသက်ရှူခက်ခြင်း, ရေဓာတ်ခန်းခြောက်မှုပြင်းထန်ခြင်း, ကင်ဆာကုသနိုင်သည်ဟု တောင်းဆိုခြင်း သို့မဟုတ် အခြားအန္တရာယ်လက္ခဏာများရှိပါက ဆေးဖက်ဝင်အပင် အကြံပြုချက်များကို ဖုံးထားသည်။',
    recordsTitle: 'စိစစ်ထားသော ဒေသတွင်းမှတ်တမ်းများ',
    recordsBody:
      'ဒီမိုတွင် source URL များ, evidence level များ, safety summary များ, contraindication များ, interaction များနှင့် ဒေသအလိုက်ရရှိနိုင်မှုပါသော SQLite dataset အသေးကို အသုံးပြုထားသည်။',
    gemmaTitle: 'Gemma အတွက် အသင့်ဖြစ်သော core',
    gemmaBody:
      'ပထမဦးဆုံး mock provider ပါဝင်သည်။ inference target အသင့်ဖြစ်လျှင် environment variables ဖြင့် HTTP Gemma endpoint သို့ ပြောင်းနိုင်သည်။',
    mobileEyebrow: 'မိုဘိုင်းနှင့် offline ဦးတည်ချက်',
    mobileTitle: 'iOS နှင့် Android ပေါ်ရှိ ဆေးဖက်ဝင်အပင် အသိပညာ',
    mobileBody:
      'Phase 2 တွင် Herb Library ကို iOS နှင့် Android ဖုန်းအက်ပ်အဖြစ် ထည့်သွင်းနိုင်ပြီး ကွန်ယက်အားနည်းသည့်အချိန်တွင်ပင် မိသားစုများနှင့် community health worker များက ဒေသအပင်များ, ဖြစ်နိုင်သော အသုံးများ, ခွဲခြားသိနိုင်သောမှတ်ချက်များ, ဘေးကင်းရေးသတိပေးချက်များနှင့် ဒေသအလိုက်ရရှိနိုင်မှုကို ကြည့်နိုင်စေသည်။',
    phoneTitle: 'ဒေသဆိုင်ရာ ဆေးဖက်ဝင်အပင် စာကြည့်တိုက်',
    phoneMoringaPack: 'Kano pack',
    phoneGingerPack: 'Bihar pack',
    phoneOffline: 'Offline အသင့်',
    offlineTitle: 'Offline-first စာကြည့်တိုက်:',
    offlineBody: 'ယုံကြည်ရသော ဒေသဆိုင်ရာ ဆေးဖက်ဝင်အပင်နှင့် အစားအစာအပင် မှတ်တမ်းများကို စက်ထဲတွင် သိမ်းထားသည်။',
    updatesTitle: 'ဒေသတွင်း အပ်ဒိတ်များ:',
    updatesBody: 'ဖုန်းပြန်ချိတ်ဆက်နိုင်သောအခါ စိစစ်ပြီးသား မှတ်တမ်းအသစ်များကို sync လုပ်သည်။',
    fieldTitle: 'ကွင်းဆင်းအသုံးပြုမှု:',
    fieldBody: 'ဘေးကင်းရေး triage, source link များနှင့် visual guide များကို လိုအပ်သူများအနီးတွင် ထားရှိသည်။'
  },
  'es-PE': {
    eyebrow: 'Navegador de conocimiento herbal con seguridad primero',
    subtitle:
      'Guía herbal local para comunidades con pocos recursos, con revisión de señales de peligro antes de mostrar cualquier planta.',
    start: 'Iniciar consulta',
    safetyPolicy: 'Leer política de seguridad',
    triageTitle: 'Triage antes de la tradición',
    triageBody:
      'Los síntomas de emergencia solo devuelven orientación para buscar atención urgente. Las plantas se ocultan cuando aparecen dolor de pecho, dificultad para respirar, deshidratación grave, pedidos de cura para cáncer u otras señales de peligro.',
    recordsTitle: 'Registros locales curados',
    recordsBody:
      'La demo usa un pequeño conjunto SQLite con URLs de fuentes, niveles de evidencia, resúmenes de seguridad, contraindicaciones, interacciones y disponibilidad regional.',
    gemmaTitle: 'Núcleo listo para Gemma',
    gemmaBody:
      'Primero se incluye un proveedor mock. Cambia a un endpoint HTTP compatible con Gemma usando variables de entorno cuando el destino de inferencia esté listo.',
    mobileEyebrow: 'Dirección móvil y sin conexión',
    mobileTitle: 'Conocimiento herbal en iOS y Android',
    mobileBody:
      'En la fase 2, la Biblioteca Herbal puede empaquetarse como app para iOS y Android, ayudando a familias y trabajadores comunitarios a revisar plantas locales, usos probables, notas de identificación, alertas de seguridad y disponibilidad regional aun con conexión débil.',
    phoneTitle: 'Biblioteca herbal regional',
    phoneMoringaPack: 'Paquete Kano',
    phoneGingerPack: 'Paquete Bihar',
    phoneOffline: 'Listo sin conexión',
    offlineTitle: 'Biblioteca offline-first:',
    offlineBody: 'guarda en el dispositivo registros confiables de hierbas regionales y plantas alimentarias.',
    updatesTitle: 'Actualizaciones locales:',
    updatesBody: 'sincroniza nuevos registros revisados cuando el teléfono vuelva a conectarse.',
    fieldTitle: 'Uso en campo:',
    fieldBody: 'mantiene triage de seguridad, enlaces de fuente y guías visuales cerca de quienes los necesitan.'
  },
  'es-BO': {
    eyebrow: 'Navegador de conocimiento herbal con seguridad primero',
    subtitle:
      'Guía herbal local para comunidades con pocos recursos, con revisión de señales de peligro antes de mostrar cualquier planta.',
    start: 'Iniciar consulta',
    safetyPolicy: 'Leer política de seguridad',
    triageTitle: 'Triage antes de la tradición',
    triageBody:
      'Los síntomas de emergencia solo devuelven orientación para buscar atención urgente. Las plantas se ocultan cuando aparecen dolor de pecho, dificultad para respirar, deshidratación grave, pedidos de cura para cáncer u otras señales de peligro.',
    recordsTitle: 'Registros locales curados',
    recordsBody:
      'La demo usa un pequeño conjunto SQLite con URLs de fuentes, niveles de evidencia, resúmenes de seguridad, contraindicaciones, interacciones y disponibilidad regional.',
    gemmaTitle: 'Núcleo listo para Gemma',
    gemmaBody:
      'Primero se incluye un proveedor mock. Cambia a un endpoint HTTP compatible con Gemma usando variables de entorno cuando el destino de inferencia esté listo.',
    mobileEyebrow: 'Dirección móvil y sin conexión',
    mobileTitle: 'Conocimiento herbal en iOS y Android',
    mobileBody:
      'En la fase 2, la Biblioteca Herbal puede empaquetarse como app para iOS y Android, ayudando a familias y trabajadores comunitarios a revisar plantas locales, usos probables, notas de identificación, alertas de seguridad y disponibilidad regional aun con conexión débil.',
    phoneTitle: 'Biblioteca herbal regional',
    phoneMoringaPack: 'Paquete Kano',
    phoneGingerPack: 'Paquete Bihar',
    phoneOffline: 'Listo sin conexión',
    offlineTitle: 'Biblioteca offline-first:',
    offlineBody: 'guarda en el dispositivo registros confiables de hierbas regionales y plantas alimentarias.',
    updatesTitle: 'Actualizaciones locales:',
    updatesBody: 'sincroniza nuevos registros revisados cuando el teléfono vuelva a conectarse.',
    fieldTitle: 'Uso en campo:',
    fieldBody: 'mantiene triage de seguridad, enlaces de fuente y guías visuales cerca de quienes los necesitan.'
  },
  'fa-AF': {
    eyebrow: 'راهنمای دانش گیاهی با ایمنی در اولویت',
    subtitle:
      'راهنمای گیاهان محلی برای جامعه‌های کم‌امکانات؛ پیش از نشان دادن هر گیاه، نشانه‌های خطر بررسی می‌شود.',
    start: 'شروع مشوره',
    safetyPolicy: 'خواندن پالیسی ایمنی',
    triageTitle: 'اول ایمنی، بعد دانش سنتی',
    triageBody:
      'در نشانه‌های عاجل فقط راهنمای مراجعه فوری داده می‌شود. اگر درد سینه، مشکل تنفس، کم‌آبی شدید، ادعای درمان سرطان یا نشانه‌های خطر دیگر باشد، پیشنهاد گیاهان پنهان می‌شود.',
    recordsTitle: 'ثبت‌های محلی بررسی‌شده',
    recordsBody:
      'این دمو از یک دیتاست کوچک SQLite استفاده می‌کند که لینک منبع، سطح شواهد، خلاصه ایمنی، منع استفاده، تداخلات و دسترسی منطقه‌ای را دارد.',
    gemmaTitle: 'هسته آماده برای Gemma',
    gemmaBody:
      'در آغاز یک mock provider همراه است. وقتی هدف inference آماده شد، با متغیرهای محیطی می‌توان به endpoint سازگار با Gemma تغییر داد.',
    mobileEyebrow: 'مسیر موبایل و آفلاین',
    mobileTitle: 'دانش گیاهی در iOS و Android',
    mobileBody:
      'در فاز ۲، Herb Library می‌تواند به شکل اپ موبایل برای iOS و Android بسته‌بندی شود تا خانواده‌ها و کارمندان صحی جامعه در اتصال ضعیف نیز گیاهان محلی، کاربردهای احتمالی، نشانه‌های شناسایی، هشدارهای ایمنی و دسترسی منطقه‌ای را ببینند.',
    phoneTitle: 'کتابخانه گیاهی منطقه‌ای',
    phoneMoringaPack: 'بسته Kano',
    phoneGingerPack: 'بسته Bihar',
    phoneOffline: 'آماده آفلاین',
    offlineTitle: 'کتابخانه آفلاین‌محور:',
    offlineBody: 'ثبت‌های قابل اعتماد گیاهان منطقه‌ای و نباتات خوراکی را روی دستگاه ذخیره می‌کند.',
    updatesTitle: 'به‌روزرسانی محلی:',
    updatesBody: 'وقتی تلفون دوباره وصل شود، ثبت‌های تازه بررسی‌شده همگام می‌شود.',
    fieldTitle: 'استفاده در ساحه:',
    fieldBody: 'triage ایمنی، لینک‌های منبع و راهنماهای تصویری را نزدیک کسانی نگه می‌دارد که به آن نیاز دارند.'
  },
  'rn-BI': {
    eyebrow: "Indongozi y'ubumenyi bw'ibimera ishira umutekano imbere",
    subtitle:
      "Ubuyobozi ku bimera vy'aho abantu baba, ku miryango ifise amikoro make, bubanza kuraba ibimenyetso vy'akaga imbere yo kwerekana igiterwa ico ari co cose.",
    start: 'Tangira inama',
    safetyPolicy: "Soma amategeko y'umutekano",
    triageTitle: 'Umutekano imbere y’imigenzo',
    triageBody:
      "Ibimenyetso vyihutirwa bitanga gusa ubuyobozi bwo kurondera ubuvuzi bwihuse. Ibimera birahishwa iyo hari ububabare bwo mu gikiriza, guhema nabi, kubura amazi cane, gusaba umuti wa kanseri, canke ibindi bimenyetso vy'akaga.",
    recordsTitle: "Inyandiko z'aho hantu zasuzumwe",
    recordsBody:
      "Demo ikoresha dataset ntoya ya SQLite irimwo amahuza y'amasoko, urugero rw'ibimenyetso, incamake y'umutekano, aho bitabereye gukoreshwa, imikoranire n'imiti, n'aho biboneka mu karere.",
    gemmaTitle: 'Umutima witeguye Gemma',
    gemmaBody:
      'Harimwo mock provider mbere. Iyo inference target yiteguye, ushobora guhindura ukaja kuri HTTP Gemma endpoint ukoresheje environment variables.',
    mobileEyebrow: 'Inzira ya telefone na offline',
    mobileTitle: 'Ubumenyi bw’ibimera kuri iOS na Android',
    mobileBody:
      "Muri Phase 2, Herb Library ishobora kuba app ya telefone kuri iOS na Android, igafasha imiryango n'abakozi b'ikibano kuraba ibimera vy'aho baba, ico bishobora gufasha, uko bobimenya, imburi z'umutekano, n'aho biboneka naho internet yaba nke.",
    phoneTitle: "Isomero ry'ibimera ry'akarere",
    phoneMoringaPack: 'Kano pack',
    phoneGingerPack: 'Bihar pack',
    phoneOffline: 'Yiteguye offline',
    offlineTitle: 'Isomero ribanza offline:',
    offlineBody: "bika kuri telefone inyandiko zizewe z'ibimera n'ibiterwa biribwa vyo mu karere.",
    updatesTitle: "Amakuru mashasha y'aho hantu:",
    updatesBody: 'telefone yongeye gufata internet, inyandiko nshasha zasuzumwe zirahuza.',
    fieldTitle: 'Gukoresha mu kibanza:',
    fieldBody: "gumana hafi y'ababikeneye triage y'umutekano, amahuza y'amasoko, n'ubuyobozi bugaragara."
  }
};

export const locale = writable<Locale>('en');

export function isLocale(value: string | null): value is Locale {
  return (
    value === 'en' ||
    value === 'sw' ||
    value === 'hi' ||
    value === 'zh' ||
    value === 'ko' ||
    value === 'vi-VN' ||
    value === 'lo-LA' ||
    value === 'my-MM' ||
    value === 'es-PE' ||
    value === 'es-BO' ||
    value === 'fa-AF' ||
    value === 'rn-BI'
  );
}
