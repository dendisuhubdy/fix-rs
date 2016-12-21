// Copyright 2016 James Bendig. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under:
//   the MIT license
//     <LICENSE-MIT or https://opensource.org/licenses/MIT>
//   or the Apache License, Version 2.0
//     <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>,
// at your option. This file may not be copied, modified, or distributed
// except according to those terms.

use dictionary::field_types::generic::{BoolTrueOrBlankFieldType,CharFieldType,DataFieldType,IntFieldType,NoneFieldType,RepeatingGroupFieldType,SeqNumFieldType,StringFieldType,UTCTimestampFieldType};
use dictionary::field_types::other as other_field_types;
use dictionary::field_types::other::{CPProgramFieldType,HandlInstFieldType,RateSourceFieldType,RateSourceTypeFieldType,SideFieldType};
use message::{REQUIRED,NOT_REQUIRED};
use rule::Rule;

//TODO: Create implementations for all of these types.
type BoolFieldType = StringFieldType;
type PercentageFieldType = StringFieldType;
type PriceFieldType = StringFieldType;
type CountryFieldType = StringFieldType;
type TZTimeOnlyFieldType = StringFieldType;
type AmtFieldType = StringFieldType;
type QtyFieldType = StringFieldType;
type LocalMktDateFieldType = StringFieldType;
type CurrencyFieldType = StringFieldType;
type MonthYearFieldType = StringFieldType;
type ExchangeFieldType = StringFieldType;
type UTCTimeOnlyFieldType = StringFieldType;

define_fields!(
    Account: StringFieldType = b"1",
    BeginSeqNo: SeqNumFieldType = b"7",
    ClOrdID: StringFieldType = b"11",
    Currency: StringFieldType = b"15", //Currency
    EndSeqNo: SeqNumFieldType = b"16",
    HandlInst: HandlInstFieldType = b"21",
    SecurityIDSource: StringFieldType = b"22", //TODO: Limited choices.
    NoLinesOfText: RepeatingGroupFieldType<LinesOfTextGrp> = b"33",
    MsgSeqNum: SeqNumFieldType = b"34", //TODO: Special field probably might be better off built into the parser.
    NewSeqNo: SeqNumFieldType = b"36",
    OrderID: StringFieldType = b"37",
    OrderQty: StringFieldType = b"38", //Qty
    OrdType: StringFieldType = b"40", //Char, TODO: limited choices.
    OrigTime: UTCTimestampFieldType = b"42",
    PossDupFlag: BoolTrueOrBlankFieldType = b"43",
    Price: StringFieldType = b"44", //Price
    RefSeqNum: SeqNumFieldType = b"45",
    SecurityID: StringFieldType = b"48", //TODO: Requires SecurityIDSource.
    SenderCompID: StringFieldType = b"49",
    SenderSubID: StringFieldType = b"50",
    SendingTime: UTCTimestampFieldType = b"52",
    SideField: SideFieldType = b"54",
    Symbol: StringFieldType = b"55",
    TargetCompID: StringFieldType = b"56",
    TargetSubID: StringFieldType = b"57",
    Text: StringFieldType = b"58",
    TimeInForce: StringFieldType = b"59", //Char, TODO: limited choices.
    TransactTime: StringFieldType = b"60", //UTCTimestamp
    SettlType: StringFieldType = b"63", //TODO: Limited choices.
    SettlDate: StringFieldType = b"64", //LocalMktDate
    SymbolSfx: StringFieldType = b"65", //TODO: Limited choices.
    NoOrders: RepeatingGroupFieldType<Order> = b"73",
    NoAllocs: RepeatingGroupFieldType<Alloc> = b"78",
    AllocAccount: StringFieldType = b"79",
    Signature: DataFieldType = b"89" => Rule::ConfirmPreviousTag{ previous_tag: SignatureLength::tag() },
    SecureDataLen: NoneFieldType = b"90" => Rule::PrepareForBytes{ bytes_tag: SecureData::tag() },
    SecureData: DataFieldType = b"91" => Rule::ConfirmPreviousTag{ previous_tag: SecureDataLen::tag() },
    SignatureLength: NoneFieldType = b"93" => Rule::PrepareForBytes{ bytes_tag: Signature::tag() },
    EmailType: CharFieldType = b"94", //TODO: Limited choices.
    RawDataLength: NoneFieldType = b"95" => Rule::PrepareForBytes{ bytes_tag: RawData::tag() },
    RawData: DataFieldType = b"96" => Rule::ConfirmPreviousTag{ previous_tag: RawDataLength::tag() },
    PossResend: StringFieldType = b"97", //Bool
    EncryptMethod: StringFieldType = b"98",
    Issuer: StringFieldType = b"106", //TODO: See Volume 7: PRODUCT: FIXED INCOME - Euro Issuer Values"
    SecurityDesc: StringFieldType = b"107",
    HeartBtInt: IntFieldType = b"108",
    MinQty: StringFieldType = b"110", //Qty
    MaxFloor: StringFieldType = b"111", //Qty
    TestReqID: StringFieldType = b"112",
    OnBehalfOfCompID: StringFieldType = b"115",
    OnBehalfOfSubID: StringFieldType = b"116",
    OrigSendingTime: UTCTimestampFieldType = b"122",
    GapFillFlag: BoolTrueOrBlankFieldType = b"123",
    DeliverToCompID: StringFieldType = b"128",
    DeliverToSubID: StringFieldType = b"129",
    BidSize: StringFieldType = b"134", //Qty
    ResetSeqNumFlag: BoolTrueOrBlankFieldType = b"141",
    SenderLocationID: StringFieldType = b"142",
    TargetLocationID: StringFieldType = b"143",
    OnBehalfOfLocationID: StringFieldType = b"144",
    DeliverToLocationID: StringFieldType = b"145",
    NoRelatedSym: RepeatingGroupFieldType<Instrument> = b"146",
    Subject: StringFieldType = b"147",
    CashOrderQty: StringFieldType = b"152", //Qty
    EmailThreadID: StringFieldType = b"164",
    SecurityType: StringFieldType = b"167", //TODO: Limited choices but needs to support user adjustable.
    MaturityMonthYear: MonthYearFieldType = b"200",
    PutOrCall: IntFieldType = b"201", //TODO: Limited choices.
    StrikePrice: StringFieldType = b"202", //Price
    OptAttribute: CharFieldType = b"206",
    SecurityExchange: ExchangeFieldType = b"207", //TODO: See Appendix 6-C for field type.
    XmlDataLen: NoneFieldType = b"212" => Rule::PrepareForBytes{ bytes_tag: XmlData::tag() },
    XmlData: DataFieldType = b"213" => Rule::ConfirmPreviousTag{ previous_tag: XmlDataLen::tag() },
    NoRoutingIDs: RepeatingGroupFieldType<RoutingGrp> = b"215",
    RoutingType: IntFieldType = b"216", //TODO: Limited choices.
    RoutingID: StringFieldType = b"217",
    CouponRate: PercentageFieldType = b"223",
    CouponPaymentDate: LocalMktDateFieldType = b"224", //TODO: Use UTCDate when FIX version < 4.4.
    IssueDate: LocalMktDateFieldType = b"225", //TODO: Use UTCDate when FIX version < 4.4.
    RepurchaseTerm: IntFieldType = b"226",
    RepurchaseRate: PercentageFieldType = b"227",
    Factor: StringFieldType = b"228", //Float
    ContractMultiplier: StringFieldType = b"231", //Float
    RepoCollateralSecurityType: StringFieldType = b"239", //TODO: Limited choices but needs to support user adjustable.
    RedemptionDate: LocalMktDateFieldType = b"240",
    UnderlyingCouponPaymentDate: LocalMktDateFieldType = b"241",
    UnderlyingIssueDate: LocalMktDateFieldType = b"242",
    UnderlyingRepoCollateralSecurityType: StringFieldType = b"243", //TODO: Limited choices but needs to support user adjustable.
    UnderlyingRepurchaseTerm: IntFieldType = b"244",
    UnderlyingRepurchaseRate: PercentageFieldType = b"245",
    UnderlyingFactor: StringFieldType = b"246", //Float
    UnderlyingRedemptionDate: LocalMktDateFieldType = b"247",
    LegCouponPaymentDate: LocalMktDateFieldType = b"248",
    LegIssueDate: LocalMktDateFieldType = b"249",
    LegRepoCollateralSecurityType: StringFieldType = b"250",
    LegRepurchaseTerm: IntFieldType = b"251",
    LegRepurchaseRate: PercentageFieldType = b"252",
    LegFactor: StringFieldType = b"253", //Float
    LegRedemptionDate: LocalMktDateFieldType = b"254",
    CreditRating: StringFieldType = b"255",
    UnderlyingCreditRating: StringFieldType = b"256",
    LegCreditRating: StringFieldType = b"257",
    UnderlyingSecurityIDSource: StringFieldType = b"305", //TODO: Limited choices.
    UnderlyingIssuer: StringFieldType = b"306", //TODO: Limited choices, maybe, see Issuer (106).
    UnderlyingSecurityDesc: StringFieldType = b"307",
    UnderlyingSecurityExchange: ExchangeFieldType = b"308",
    UnderlyingSecurityID: StringFieldType = b"309", //TODO: Required UnderlyingSecurityIDSource.
    UnderlyingSecurityType: StringFieldType = b"310", //TODO: Limited choices but needs to support user adjustable.
    UnderlyingSymbol: StringFieldType = b"311",
    UnderlyingSymbolSfx: StringFieldType = b"312", //TODO: Limited choices.
    UnderlyingMaturityMonthYear: MonthYearFieldType = b"313",
    UnderlyingPutOrCall: IntFieldType = b"315", //TODO: Limited choices.
    UnderlyingStrikePrice: StringFieldType = b"316", //Price
    UnderlyingOptAttribute: CharFieldType = b"317",
    UnderlyingCurrency: CurrencyFieldType = b"318",
    MessageEncoding: StringFieldType = b"347",
    EncodedIssuerLen: NoneFieldType = b"348" => Rule::PrepareForBytes{ bytes_tag: EncodedIssuer::tag() },
    EncodedIssuer: DataFieldType = b"349" => Rule::ConfirmPreviousTag{ previous_tag: EncodedIssuerLen::tag() },
    EncodedSecurityDescLen: NoneFieldType = b"350" => Rule::PrepareForBytes{ bytes_tag: EncodedSecurityDesc::tag() },
    EncodedSecurityDesc: DataFieldType = b"351" => Rule::ConfirmPreviousTag{ previous_tag: EncodedSecurityDescLen::tag() },
    EncodedTextLen: NoneFieldType = b"354" => Rule::PrepareForBytes{ bytes_tag: EncodedText::tag() },
    EncodedText: DataFieldType = b"355" => Rule::ConfirmPreviousTag{ previous_tag: EncodedTextLen::tag() },
    EncodedSubjectLen: NoneFieldType = b"356" => Rule::PrepareForBytes{ bytes_tag: EncodedSubject::tag() },
    EncodedSubject: DataFieldType = b"357" => Rule::ConfirmPreviousTag{ previous_tag: EncodedSubjectLen::tag() },
    EncodedUnderlyingIssuerLen: NoneFieldType = b"362" => Rule::PrepareForBytes{ bytes_tag: EncodedUnderlyingIssuer::tag() },
    EncodedUnderlyingIssuer: DataFieldType = b"363" => Rule::ConfirmPreviousTag{ previous_tag: EncodedUnderlyingIssuerLen::tag() },
    EncodedUnderlyingSecurityDescLen: NoneFieldType = b"364" => Rule::PrepareForBytes{ bytes_tag: EncodedUnderlyingSecurityDesc::tag() },
    EncodedUnderlyingSecurityDesc: DataFieldType = b"365" => Rule::ConfirmPreviousTag{ previous_tag: EncodedUnderlyingSecurityDescLen::tag() },
    LastMsgSeqNumProcessed: SeqNumFieldType = b"369",
    RefTagID: StringFieldType = b"371", //int
    RefMsgType: StringFieldType = b"372",
    SessionRejectReason: StringFieldType = b"373", //TODO: Use SessionRejectReasonFieldType. Too many unreleated corrections to change this now.
    BusinessRejectRefID: StringFieldType = b"379",
    BusinessRejectReason: StringFieldType = b"380", //int //TODO: limited choices.
    MaxMessageSize: StringFieldType = b"383", //Length
    NoMsgTypeGrp: RepeatingGroupFieldType<MsgTypeGrp> = b"384",
    MsgDirection: StringFieldType = b"385", //Char
    UnderlyingCouponRate: PercentageFieldType = b"435",
    UnderlyingContractMultiplier: StringFieldType = b"436", //Float
    NoSecurityAltID: RepeatingGroupFieldType<SecAltIDGrp> = b"454",
    SecurityAltID: StringFieldType = b"455",
    SecurityAltIDSource: StringFieldType = b"456", //TODO: Limited choices.
    NoUnderlyingSecurityAltID: RepeatingGroupFieldType<UndSecAltIDGrp> = b"457",
    UnderlyingSecurityAltID: StringFieldType = b"458", //TODO: Requires UnderlyingSecurityAltIDSource.
    UnderlyingSecurityAltIDSource: StringFieldType = b"459", //TODO: Limited choices.
    Product: IntFieldType = b"460", //TODO: Limited choices.
    CFICode: StringFieldType = b"461",
    UnderlyingProduct: IntFieldType = b"462", //TODO: Limited choices.
    UnderlyingCFICode: StringFieldType = b"463",
    TestMessageIndicator: StringFieldType = b"464", //Bool
    CountryOfIssue: CountryFieldType = b"470",
    StateOrProvinceOfIssue: StringFieldType = b"471",
    LocaleOfIssue: StringFieldType = b"472", //TODO: Limited choices.
    MaturityDate: LocalMktDateFieldType = b"541",
    UnderlyingMaturityDate: LocalMktDateFieldType = b"542",
    InstrRegistry: StringFieldType = b"543",
    Username: StringFieldType = b"553",
    Password: StringFieldType = b"554",
    NoLegs: RepeatingGroupFieldType<InstrumentLeg> = b"555",
    LegCurrency: CurrencyFieldType = b"556",
    LegPrice: PriceFieldType = b"566",
    UnderlyingCountryOfIssue: CountryFieldType = b"592",
    UnderlyingStateOrProvinceOfIssue: StringFieldType = b"593",
    UnderlyingLocaleOfIssue: StringFieldType = b"594", //TODO: Limited choices.
    UnderlyingInstrRegistry: StringFieldType = b"595",
    LegCountryOfIssue: CountryFieldType = b"596",
    LegStateOrProvinceOfIssue: StringFieldType = b"597",
    LegLocaleOfIssue: StringFieldType = b"598", //TODO: Limited choices.
    LegInstrRegistry: StringFieldType = b"599",
    LegSymbol: StringFieldType = b"600",
    LegSymbolSfx: StringFieldType = b"601", //TODO: Limited choices.
    LegSecurityID: StringFieldType = b"602", //TODO: Requires LegSecurityIDSource
    LegSecurityIDSource: StringFieldType = b"603", //TODO: Limited choices,
    NoLegSecurityAltID: RepeatingGroupFieldType<LegSecAltIDGrp> = b"604",
    LegSecurityAltID: StringFieldType = b"605",
    LegSecurityAltIDSource: StringFieldType = b"606", //TODO: Limited choices.
    LegProduct: IntFieldType = b"607", //TODO: Limited choices.
    LegCFICode: StringFieldType = b"608",
    LegSecurityType: StringFieldType = b"609", //TODO: Limited choices.
    LegMaturityMonthYear: MonthYearFieldType = b"610",
    LegMaturityDate: LocalMktDateFieldType = b"611",
    LegStrikePrice: PriceFieldType = b"612",
    LegOptAttribute: CharFieldType = b"613",
    LegContractMultiplier: StringFieldType = b"614", //Float
    LegCouponRate: PercentageFieldType = b"615",
    LegSecurityExchange: ExchangeFieldType = b"616",
    LegIssuer: StringFieldType = b"617", //TODO: See Issuer (106).
    EncodedLegIssuerLen: NoneFieldType = b"618" => Rule::PrepareForBytes{ bytes_tag: EncodedLegIssuer::tag() },
    EncodedLegIssuer: DataFieldType = b"619" => Rule::ConfirmPreviousTag{ previous_tag: EncodedLegIssuerLen::tag() },
    LegSecurityDesc: StringFieldType = b"620",
    EncodedLegSecurityDescLen: NoneFieldType = b"621" => Rule::PrepareForBytes{ bytes_tag: EncodedLegSecurityDesc::tag() },
    EncodedLegSecurityDesc: DataFieldType = b"622" => Rule::ConfirmPreviousTag{ previous_tag: EncodedLegSecurityDescLen::tag() },
    LegRatioQty: StringFieldType = b"623", //Float
    LegSide: CharFieldType = b"624", //TODO: Limited choices.
    NoHops: RepeatingGroupFieldType<HopGrp> = b"627",
    HopCompID: StringFieldType = b"628",
    HopSendingTime: StringFieldType = b"629", //UTCTimestamp
    HopRefID: SeqNumFieldType = b"630",
    ContractSettlMonth: MonthYearFieldType = b"667",
    Pool: StringFieldType = b"691",
    NoUnderlyings: RepeatingGroupFieldType<UnderlyingInstrument> = b"711",
    LegDatedDate: LocalMktDateFieldType = b"739",
    LegPool: StringFieldType = b"740",
    SecuritySubType: StringFieldType = b"762",
    UnderlyingSecuritySubType: StringFieldType = b"763",
    LegSecuritySubType: StringFieldType = b"764",
    NextExpectedMsgSeqNum: SeqNumFieldType = b"789",
    UnderlyingPx: PriceFieldType = b"810",
    NoEvents: RepeatingGroupFieldType<EvntGrp> = b"864",
    EventType: IntFieldType = b"865", //TODO: Limited choices.
    EventDate: LocalMktDateFieldType = b"866",
    EventPx: PriceFieldType = b"867",
    EventText: StringFieldType = b"868",
    DatedDate: LocalMktDateFieldType = b"873",
    InterestAccrualDate: LocalMktDateFieldType = b"874",
    CPProgram: CPProgramFieldType = b"875",
    CPRegType: StringFieldType = b"876",
    UnderlyingCPProgram: CPProgramFieldType = b"877",
    UnderlyingCPRegType: StringFieldType = b"878",
    UnderlyingQty: QtyFieldType = b"879",
    UnderlyingDirtyPrice: PriceFieldType = b"882",
    UnderlyingEndPrice: PriceFieldType = b"883",
    UnderlyingStartValue: AmtFieldType = b"884",
    UnderlyingCurrentValue: AmtFieldType = b"885",
    UnderlyingEndValue: AmtFieldType = b"886",
    NoUnderlyingStips: RepeatingGroupFieldType<UnderlyingStipulation> = b"887",
    UnderlyingStipType: StringFieldType = b"888", //TODO: Limited choices
    UnderlyingStipValue: StringFieldType = b"889", //TODO: Parsable expression.
    NewPassword: StringFieldType = b"925",
    UnderlyingStrikeCurrency: CurrencyFieldType = b"941",
    LegStrikeCurrency: CurrencyFieldType = b"942",
    StrikeCurrency: CurrencyFieldType = b"947",
    LegContractSettlMonth: MonthYearFieldType = b"955",
    LegInterestAccrualDate: LocalMktDateFieldType = b"956",
    SecurityStatus: StringFieldType = b"965", //TODO: Limited choices.
    SettleOnOpenFlag: StringFieldType = b"966",
    StrikeMultiplier: StringFieldType = b"967", //Float
    StrikeValue: StringFieldType = b"968", //Float
    MinPriceIncrement: StringFieldType = b"969", //Float
    PositionLimit: IntFieldType = b"970",
    NTPositionLimit: IntFieldType = b"971",
    UnderlyingAllocationPercent: PercentageFieldType = b"972",
    UnderlyingCashAmount: AmtFieldType = b"973",
    UnderlyingCashType: StringFieldType = b"974", //TODO: Limited choices.
    UnderlyingSettlementType: IntFieldType = b"975", //TODO: Limited choices, maybe?
    UnitOfMeasure: StringFieldType = b"996", //TODO: Limited choices.
    TimeUnit: StringFieldType = b"997", //TODO: Limited choices with third party choices support.
    UnderlyingUnitOfMeasure: StringFieldType = b"998", //TODO: Limited choices.
    LegUnitOfMeasure: StringFieldType = b"999", //TODO: Limited choices.
    UnderlyingTimeUnit: StringFieldType = b"1000", //TODO: Limited choices.
    LegTimeUnit: StringFieldType = b"1001", //TODO: Limited choices.
    LegOptionRatio: StringFieldType = b"1017", //Float
    NoInstrumentParties: RepeatingGroupFieldType<InstrumentParty> = b"1018",
    InstrumentPartyID: StringFieldType = b"1019", //TODO: Limited choices. See PartyID (448).
    UnderlyingDeliveryAmount: AmtFieldType = b"1037",
    UnderlyingCapValue: AmtFieldType = b"1038",
    UnderlyingSettlMethod: StringFieldType = b"1039",
    UnderlyingAdjustedQuantity: StringFieldType = b"1044", //Qty
    UnderlyingFXRate: StringFieldType = b"1045", //Float
    UnderlyingFXRateCalc: CharFieldType = b"1046", //TODO: Limited choices.
    NoUndlyInstrumentParties: RepeatingGroupFieldType<UndlyInstrumentPtysSubGrp> = b"1058",
    InstrmtAssignmentMethod: CharFieldType = b"1049", //TODO: Limited choices.
    InstrumentPartyIDSource: CharFieldType = b"1050", //TODO: Limited choices.
    InstrumentPartyRole: IntFieldType = b"1051", //TODO: Limited choices.
    NoInstrumentPartySubIDs: RepeatingGroupFieldType<InstrumentPtysSubGrp> = b"1052",
    InstrumentPartySubID: StringFieldType = b"1053",
    InstrumentPartySubIDType: IntFieldType = b"1054", //TODO: Limited choices.
    NoUndlyInstrumentPartySubIDs: RepeatingGroupFieldType<UndlyInstrumentPtysSubGrp> = b"1062",
    UnderlyingInstrumentPartySubID: StringFieldType = b"1063",
    UnderlyingInstrumentPartySubIDType: IntFieldType = b"1064", //TODO: Limited choices.
    MaturityTime: TZTimeOnlyFieldType = b"1079",
    ApplVerID: StringFieldType = b"1128", //TODO: limited choices.
    CstmApplVerID: StringFieldType = b"1129",
    RefApplVerID: StringFieldType = b"1130",
    RefCstmApplVerID: StringFieldType = b"1131",
    DefaultApplVerID: StringFieldType = b"1137", //TODO: limited choices.
    EventTime: UTCTimestampFieldType = b"1145",
    MinPriceIncrementAmount: AmtFieldType = b"1146",
    UnitOfMeasureQty: StringFieldType = b"1147", //Qty
    SecurityGroup: StringFieldType = b"1151",
    ApplExtID: StringFieldType = b"1156", //int
    SecurityXMLLen: NoneFieldType = b"1184" => Rule::PrepareForBytes{ bytes_tag: SecurityXML::tag() },
    SecurityXML: DataFieldType = b"1185" => Rule::ConfirmPreviousTag{ previous_tag: SecurityXMLLen::tag() },
    SecurityXMLSchema: StringFieldType = b"1186",
    PriceUnitOfMeasure: StringFieldType = b"1191", //TODO: Limited choices.
    PriceUnitOfMeasureQty: StringFieldType = b"1192", //Qty
    SettlMethod: CharFieldType = b"1193", //TODO: Limited choices.
    ExerciseStyle: IntFieldType = b"1194", //TODO: Limited choices.
    OptPayoutAmount: AmtFieldType = b"1195", //TODO: Conditionally required if OptPayoutType is set to binary.
    PriceQuoteMethod: StringFieldType = b"1196", //TODO: Limited choices.
    ValuationMethod: StringFieldType = b"1197", //TODO: Limited choices.
    ListMethod: IntFieldType = b"1198", //TODO: Limited choices.
    CapPrice: PriceFieldType = b"1199",
    FloorPrice: PriceFieldType = b"1200",
    LegMaturityTime: TZTimeOnlyFieldType = b"1212",
    UnderlyingMaturityTime: TZTimeOnlyFieldType = b"1213",
    LegUnitOfMeasureQty: StringFieldType = b"1224", //Qty
    ProductComplex: StringFieldType = b"1227",
    FlexibleProductElgibilityIndicator: BoolFieldType = b"1242",
    FlexibleIndicator: BoolFieldType = b"1244",
    LegPutOrCall: IntFieldType = b"1358", //TODO: Limited choices.
    EncryptedPasswordMethod: StringFieldType = b"1400", //int
    EncryptedPasswordLen: NoneFieldType = b"1401" => Rule::PrepareForBytes{ bytes_tag: EncryptedPassword::tag() },
    EncryptedPassword: DataFieldType = b"1402" => Rule::ConfirmPreviousTag{ previous_tag: EncryptedPasswordLen::tag() },
    EncryptedNewPasswordLen: NoneFieldType = b"1403" => Rule::PrepareForBytes{ bytes_tag: EncryptedNewPassword::tag() },
    EncryptedNewPassword: DataFieldType = b"1404" => Rule::ConfirmPreviousTag{ previous_tag: EncryptedNewPasswordLen::tag() },
    RefApplExtID: StringFieldType = b"1406", //int
    DefaultApplExtID: StringFieldType = b"1407", //int
    DefaultCstmApplVerID: StringFieldType = b"1408",
    SessionStatus: StringFieldType = b"1409", //int
    DefaultVerIndicator: StringFieldType = b"1410", //bool
    UnderlyingExerciseStyle: IntFieldType = b"1419", //TODO: Limited choices.
    LegExerciseStyle: IntFieldType = b"1420", //TODO: Limited choices.
    LegPriceUnitOfMeasure: StringFieldType = b"1421", //TODO: Limited choices.
    LegPriceUnitOfMeasureQty: StringFieldType = b"1422", //Qty
    UnderlyingUnitOfMeasureQty: StringFieldType = b"1423", //Qty
    UnderlyingPriceUnitOfMeasure: StringFieldType = b"1424", //TODO: Limted choices.
    UnderlyingPriceUnitOfMeasureQty: StringFieldType = b"1425", //Qty
    ContractMultiplierUnit: IntFieldType = b"1435", //TODO: Limited choices.
    LegContractMultiplierUnit: IntFieldType = b"1436", //TODO: Limited choices.
    UnderlyingContractMultiplierUnit: IntFieldType = b"1437", //TODO: Limited choices.
    FlowScheduleType: IntFieldType = b"1439", //TODO: Limited choices plus user choice.
    LegFlowScheduleType: IntFieldType = b"1440", //TODO: Limited choices plus user choice.
    UnderlyingFlowScheduleType: IntFieldType = b"1441", //TODO: Limited choices plus user choice.
    NoRateSources: RepeatingGroupFieldType<RateSourceGrp> = b"1445",
    RateSource: RateSourceFieldType = b"1446",
    RateSourceType: RateSourceTypeFieldType = b"1447",
    ReferencePage: StringFieldType = b"1448",
    RestructuringType: StringFieldType = b"1449", //TODO: Limited choices.
    Seniority: StringFieldType = b"1450", //TODO: Limited choices.
    NotionalPercentageOutstanding: PercentageFieldType = b"1451",
    OriginalNotionalPercentageOutstanding: PercentageFieldType = b"1452",
    UnderlyingRestructuringType: StringFieldType = b"1453", //TODO: Limited choices,
    UnderlyingSeniority: StringFieldType = b"1454", //TODO: Limited choices.
    UnderlyingNotionalPercentageOutstanding: PercentageFieldType = b"1455",
    UnderlyingOriginalNotionalPercentageOutstanding: PercentageFieldType = b"1456",
    AttachmentPoint: PercentageFieldType = b"1457",
    DetachmentPoint: PercentageFieldType = b"1458",
    UnderlyingAttachmentPoint: PercentageFieldType = b"1459",
    UnderlyingDetachmentPoint: PercentageFieldType = b"1460",
    StrikePriceDeterminationMethod: IntFieldType = b"1478", //TODO: Limited choices but user choices supported.
    StrikePriceBoundaryMethod: IntFieldType = b"1479", //TODO: Limited choices.
    StrikePriceBoundaryPrecision: PercentageFieldType = b"1480",
    UnderlyingPriceDeterminationMethod: IntFieldType = b"1481", //TODO: Limited choices.
    OptPayoutType: IntFieldType = b"1482", //TODO: Limited choices.
    NoComplexEvents: RepeatingGroupFieldType<ComplexEvent> = b"1483",
    ComplexEventType: IntFieldType = b"1484", //TODO: Limited choices
    ComplexOptPayoutAmount: AmtFieldType = b"1485",
    ComplexEventPrice: PriceFieldType = b"1486",
    ComplexEventPriceBoundaryMethod: IntFieldType = b"1487", //TODO: Limited choices.
    ComplexEventPriceBoundaryPrecision: PercentageFieldType = b"1488",
    ComplexEventPriceTimeType: IntFieldType = b"1489", //TODO: Limited choices.
    ComplexEventCondition: IntFieldType = b"1490", //TODO: Limited choices.
    NoComplexEventDates: RepeatingGroupFieldType<ComplexEventDate> = b"1491",
    ComplexEventStartDate: UTCTimestampFieldType = b"1492", //TODO: Must always be less than end date.
    ComplexEventEndDate: UTCTimestampFieldType = b"1493", //TODO: Must always be greater than event start date.
    NoComplexEventTimes: RepeatingGroupFieldType<ComplexEventTime> = b"1494",
    ComplexEventStartTime: UTCTimeOnlyFieldType = b"1495", //TODO: Must always be less than end time.
    ComplexEventEndTime: UTCTimeOnlyFieldType = b"1496", //TODO: Must always be greater than start time.
);

//Repeating Groups (Sorted Alphabetically)

define_message!(Alloc {
    REQUIRED, alloc_account: AllocAccount,
});

define_message!(ComplexEvent {
    REQUIRED, complex_event_type: ComplexEventType,
    NOT_REQUIRED, complex_opt_payout_amount: ComplexOptPayoutAmount,
    NOT_REQUIRED, complex_event_price: ComplexEventPrice,
    NOT_REQUIRED, complex_event_price_boundary_method: ComplexEventPriceBoundaryMethod,
    NOT_REQUIRED, complex_event_price_boundary_precision: ComplexEventPriceBoundaryPrecision,
    NOT_REQUIRED, complex_event_price_time_type: ComplexEventPriceTimeType,
    NOT_REQUIRED, complex_event_condition: ComplexEventCondition, //TODO: Conditionally required only when there is more than one ComplexEvent.
    NOT_REQUIRED, no_complex_event_dates: NoComplexEventDates,
});

define_message!(ComplexEventDate {
    REQUIRED, complex_event_start_date: ComplexEventStartDate,
    REQUIRED, complex_event_end_date: ComplexEventEndDate,
    NOT_REQUIRED, no_complex_event_times: NoComplexEventTimes,
});

define_message!(ComplexEventTime {
    REQUIRED, complex_event_start_time: ComplexEventStartTime,
    REQUIRED, complex_event_end_time: ComplexEventEndTime,
});

define_message!(EvntGrp {
    REQUIRED, event_type: EventType,
    NOT_REQUIRED, event_date: EventDate,
    NOT_REQUIRED, event_time: EventTime,
    NOT_REQUIRED, event_px: EventPx,
    NOT_REQUIRED, event_text: EventText,
});

define_message!(HopGrp {
    REQUIRED, hop_comp_id: HopCompID,
    NOT_REQUIRED, hop_sending_time: HopSendingTime,
    NOT_REQUIRED, hop_ref_id: HopRefID,
});

define_message!(Instrument {
    REQUIRED, symbol: Symbol,
    NOT_REQUIRED, symbol_sfx: SymbolSfx,
    NOT_REQUIRED, security_id: SecurityID,
    NOT_REQUIRED, security_id_source: SecurityIDSource => EXCEPT_WHEN message, !message.security_id.is_empty(),
    NOT_REQUIRED, no_security_alt_id: NoSecurityAltID,
    NOT_REQUIRED, product: Product,
    NOT_REQUIRED, product_complex: ProductComplex,
    NOT_REQUIRED, security_group: SecurityGroup,
    NOT_REQUIRED, cfi_code: CFICode,
    NOT_REQUIRED, security_type: SecurityType => EXCEPT_WHEN message, !message.security_sub_type.is_empty(),
    NOT_REQUIRED, security_sub_type: SecuritySubType,
    NOT_REQUIRED, maturity_month_year: MaturityMonthYear,
    NOT_REQUIRED, maturity_date: MaturityDate,
    NOT_REQUIRED, maturity_time: MaturityTime,
    NOT_REQUIRED, settle_on_open_flag: SettleOnOpenFlag,
    NOT_REQUIRED, instrmt_assignment_method: InstrmtAssignmentMethod,
    NOT_REQUIRED, security_status: SecurityStatus,
    NOT_REQUIRED, coupon_payment_date: CouponPaymentDate,
    NOT_REQUIRED, restructuring_type: RestructuringType,
    NOT_REQUIRED, seniority: Seniority,
    NOT_REQUIRED, notional_percentage_outstanding: NotionalPercentageOutstanding,
    NOT_REQUIRED, original_notional_percentage_outstanding: OriginalNotionalPercentageOutstanding,
    NOT_REQUIRED, attachment_point: AttachmentPoint,
    NOT_REQUIRED, detachment_point: DetachmentPoint,
    NOT_REQUIRED, issue_date: IssueDate,
    NOT_REQUIRED, repo_collateral_security_type: RepoCollateralSecurityType,
    NOT_REQUIRED, repurchase_term: RepurchaseTerm,
    NOT_REQUIRED, repurchase_rate: RepurchaseRate,
    NOT_REQUIRED, factor: Factor,
    NOT_REQUIRED, credit_rating: CreditRating,
    NOT_REQUIRED, instr_registry: InstrRegistry,
    NOT_REQUIRED, country_of_issue: CountryOfIssue,
    NOT_REQUIRED, state_or_province_of_issue: StateOrProvinceOfIssue,
    NOT_REQUIRED, locale_of_issue: LocaleOfIssue,
    NOT_REQUIRED, redemption_date: RedemptionDate,
    NOT_REQUIRED, strike_price: StrikePrice,
    NOT_REQUIRED, strike_currency: StrikeCurrency,
    NOT_REQUIRED, strike_multiplier: StrikeMultiplier,
    NOT_REQUIRED, strike_value: StrikeValue,
    NOT_REQUIRED, strike_price_determination_method: StrikePriceDeterminationMethod,
    NOT_REQUIRED, strike_price_boundary_method: StrikePriceBoundaryMethod,
    NOT_REQUIRED, strike_price_boundary_precision: StrikePriceBoundaryPrecision,
    NOT_REQUIRED, underlying_price_determination_method: UnderlyingPriceDeterminationMethod,
    NOT_REQUIRED, opt_attribute: OptAttribute,
    NOT_REQUIRED, contract_multiplier: ContractMultiplier,
    NOT_REQUIRED, contract_multiplier_unit: ContractMultiplierUnit,
    NOT_REQUIRED, flow_schedule_type: FlowScheduleType,
    NOT_REQUIRED, min_price_increment: MinPriceIncrement,
    NOT_REQUIRED, min_price_increment_amount: MinPriceIncrementAmount,
    NOT_REQUIRED, unit_of_measure: UnitOfMeasure,
    NOT_REQUIRED, unit_of_measure_qty: UnitOfMeasureQty,
    NOT_REQUIRED, price_unit_of_measure: PriceUnitOfMeasure,
    NOT_REQUIRED, price_unit_of_measure_qty: PriceUnitOfMeasureQty,
    NOT_REQUIRED, settl_method: SettlMethod,
    NOT_REQUIRED, exercise_style: ExerciseStyle,
    NOT_REQUIRED, opt_payout_type: OptPayoutType,
    NOT_REQUIRED, opt_payout_amount: OptPayoutAmount,
    NOT_REQUIRED, price_quote_method: PriceQuoteMethod,
    NOT_REQUIRED, valuation_method: ValuationMethod,
    NOT_REQUIRED, list_method: ListMethod,
    NOT_REQUIRED, cap_price: CapPrice,
    NOT_REQUIRED, floor_price: FloorPrice,
    NOT_REQUIRED, put_or_call: PutOrCall,
    NOT_REQUIRED, flexible_indicator: FlexibleIndicator,
    NOT_REQUIRED, flexible_product_eligibility_indicator: FlexibleProductElgibilityIndicator,
    NOT_REQUIRED, time_unit: TimeUnit,
    NOT_REQUIRED, coupon_rate: CouponRate,
    NOT_REQUIRED, security_exchange: SecurityExchange,
    NOT_REQUIRED, position_limit: PositionLimit,
    NOT_REQUIRED, nt_position_limit: NTPositionLimit,
    NOT_REQUIRED, issuer: Issuer,
    NOT_REQUIRED, encoded_issuer_len: EncodedIssuerLen,
    NOT_REQUIRED, encoded_issuer: EncodedIssuer,
    NOT_REQUIRED, security_desc: SecurityDesc,
    NOT_REQUIRED, encoded_security_desc_len: EncodedSecurityDescLen,
    NOT_REQUIRED, encoded_security_desc: EncodedSecurityDesc,
    NOT_REQUIRED, security_xml_len: SecurityXMLLen,
    NOT_REQUIRED, security_xml: SecurityXML,
    NOT_REQUIRED, security_xml_schema: SecurityXMLSchema,
    NOT_REQUIRED, pool: Pool,
    NOT_REQUIRED, contract_settl_month: ContractSettlMonth,
    NOT_REQUIRED, cp_program: CPProgram,
    NOT_REQUIRED, cp_reg_type: CPRegType,
    NOT_REQUIRED, no_events: NoEvents,
    NOT_REQUIRED, dated_date: DatedDate,
    NOT_REQUIRED, interest_accrual_date: InterestAccrualDate,
    NOT_REQUIRED, no_instrument_parties: NoInstrumentParties,
    NOT_REQUIRED, no_complex_events: NoComplexEvents,
});

define_message!(InstrumentLeg {
    REQUIRED, leg_symbol: LegSymbol,
    NOT_REQUIRED, leg_symbol_sfx: LegSymbolSfx,
    NOT_REQUIRED, leg_security_id: LegSecurityID,
    NOT_REQUIRED, leg_security_id_source: LegSecurityIDSource => EXCEPT_WHEN message, !message.leg_security_id.is_empty(),
    NOT_REQUIRED, no_leg_security_alt_id: NoLegSecurityAltID,
    NOT_REQUIRED, leg_product: LegProduct,
    NOT_REQUIRED, leg_cfi_code: LegCFICode,
    NOT_REQUIRED, leg_security_type: LegSecurityType => EXCEPT_WHEN message, !message.leg_security_sub_type.is_empty(),
    NOT_REQUIRED, leg_security_sub_type: LegSecuritySubType,
    NOT_REQUIRED, leg_maturity_month_year: LegMaturityMonthYear,
    NOT_REQUIRED, leg_maturity_date: LegMaturityDate,
    NOT_REQUIRED, leg_maturity_time: LegMaturityTime,
    NOT_REQUIRED, leg_coupon_payment_date: LegCouponPaymentDate,
    NOT_REQUIRED, leg_issue_date: LegIssueDate,
    NOT_REQUIRED, leg_repo_collateral_security_type: LegRepoCollateralSecurityType,
    NOT_REQUIRED, leg_repurchase_term: LegRepurchaseTerm,
    NOT_REQUIRED, leg_repurchase_rate: LegRepurchaseRate,
    NOT_REQUIRED, leg_factor: LegFactor,
    NOT_REQUIRED, leg_credit_rating: LegCreditRating,
    NOT_REQUIRED, leg_instr_registry: LegInstrRegistry,
    NOT_REQUIRED, leg_country_of_issue: LegCountryOfIssue,
    NOT_REQUIRED, leg_state_or_province_of_issue: LegStateOrProvinceOfIssue,
    NOT_REQUIRED, leg_locale_of_issue: LegLocaleOfIssue,
    NOT_REQUIRED, leg_redemption_date: LegRedemptionDate,
    NOT_REQUIRED, leg_strike_price: LegStrikePrice,
    NOT_REQUIRED, leg_strike_currency: LegStrikeCurrency,
    NOT_REQUIRED, leg_opt_attribute: LegOptAttribute,
    NOT_REQUIRED, leg_contract_multiplier: LegContractMultiplier,
    NOT_REQUIRED, leg_contract_multiplier_unit: LegContractMultiplierUnit,
    NOT_REQUIRED, leg_flow_schedule_type: LegFlowScheduleType,
    NOT_REQUIRED, leg_unit_of_measure: LegUnitOfMeasure,
    NOT_REQUIRED, leg_unit_of_measure_qty: LegUnitOfMeasureQty,
    NOT_REQUIRED, leg_price_unit_of_measure: LegPriceUnitOfMeasure,
    NOT_REQUIRED, leg_price_unit_of_measure_qty: LegPriceUnitOfMeasureQty,
    NOT_REQUIRED, leg_time_unit: LegTimeUnit,
    NOT_REQUIRED, leg_exercise_style: LegExerciseStyle,
    NOT_REQUIRED, leg_coupon_rate: LegCouponRate,
    NOT_REQUIRED, leg_security_exchange: LegSecurityExchange,
    NOT_REQUIRED, leg_issuer: LegIssuer,
    NOT_REQUIRED, encoded_leg_issuer_len: EncodedLegIssuerLen,
    NOT_REQUIRED, encoded_leg_issuer: EncodedLegIssuer,
    NOT_REQUIRED, leg_security_desc: LegSecurityDesc,
    NOT_REQUIRED, encoded_leg_security_desc_len: EncodedLegSecurityDescLen,
    NOT_REQUIRED, encoded_leg_security_desc: EncodedLegSecurityDesc,
    NOT_REQUIRED, leg_ratio_qty: LegRatioQty,
    NOT_REQUIRED, leg_side: LegSide,
    NOT_REQUIRED, leg_currency: LegCurrency,
    NOT_REQUIRED, leg_poll: LegPool,
    NOT_REQUIRED, leg_dated_date: LegDatedDate,
    NOT_REQUIRED, leg_contract_settl_month: LegContractSettlMonth,
    NOT_REQUIRED, leg_interest_accrual_date: LegInterestAccrualDate,
    NOT_REQUIRED, leg_put_or_call: LegPutOrCall,
    NOT_REQUIRED, leg_option_ratio: LegOptionRatio,
    NOT_REQUIRED, leg_price: LegPrice,
});

define_message!(InstrumentParty {
    REQUIRED, instrument_party_id: InstrumentPartyID,
    NOT_REQUIRED, instrument_party_id_source: InstrumentPartyIDSource,
    NOT_REQUIRED, instrument_party_role: InstrumentPartyRole,
    NOT_REQUIRED, no_instrument_party_sub_ids: NoInstrumentPartySubIDs,
});

define_message!(InstrumentPtysSubGrp {
    REQUIRED, instrument_party_sub_id: InstrumentPartySubID,
    REQUIRED, instrument_party_sub_id_type: InstrumentPartySubIDType,
});

define_message!(LegSecAltIDGrp {
    REQUIRED, leg_security_alt_id: LegSecurityAltID,
    REQUIRED, leg_security_alt_id_source: LegSecurityAltIDSource,
});

define_message!(LinesOfTextGrp {
    REQUIRED, text: Text,
    NOT_REQUIRED, encoded_text_len: EncodedTextLen,
    NOT_REQUIRED, encoded_text: EncodedText,
});

define_message!(MsgTypeGrp {
    REQUIRED, ref_msg_type: RefMsgType,
    REQUIRED, msg_direction: MsgDirection,
    NOT_REQUIRED, ref_appl_ver_id: RefApplVerID,
    NOT_REQUIRED, ref_appl_ext_id: RefApplExtID,
    NOT_REQUIRED, ref_cstm_appl_ver_id: RefCstmApplVerID,
    NOT_REQUIRED, default_ver_indicator: DefaultVerIndicator,
});

define_message!(Order {
    REQUIRED, cl_ord_id: ClOrdID,
    NOT_REQUIRED, allocs: NoAllocs,
});

define_message!(RateSourceGrp {
    REQUIRED, rate_source: RateSource,
    REQUIRED, rate_source_type: RateSourceType,
    NOT_REQUIRED, reference_page: ReferencePage => EXCEPT_WHEN message, message.rate_source == other_field_types::RateSource::Other,
});

define_message!(RoutingGrp {
    REQUIRED, routing_type: RoutingType,
    REQUIRED, routing_id: RoutingID,
});

define_message!(SecAltIDGrp {
    REQUIRED, security_alt_id: SecurityAltID,
    REQUIRED, security_alt_id_source: SecurityAltIDSource,
});

define_message!(UnderlyingInstrument {
    REQUIRED, underlying_symbol: UnderlyingSymbol,
    NOT_REQUIRED, underlying_symbol_sfx: UnderlyingSymbolSfx,
    NOT_REQUIRED, underlying_security_id: UnderlyingSecurityID,
    NOT_REQUIRED, underlying_security_id_source: UnderlyingSecurityIDSource => EXCEPT_WHEN message, !message.underlying_security_id.is_empty(),
    NOT_REQUIRED, no_underlying_security_alt_id: NoUnderlyingSecurityAltID,
    NOT_REQUIRED, underlying_product: UnderlyingProduct,
    NOT_REQUIRED, underlying_cfi_code: UnderlyingCFICode,
    NOT_REQUIRED, underlying_security_type: UnderlyingSecurityType => EXCEPT_WHEN message, !message.underlying_security_sub_type.is_empty(),
    NOT_REQUIRED, underlying_security_sub_type: UnderlyingSecuritySubType,
    NOT_REQUIRED, underlying_maturity_month_year: UnderlyingMaturityMonthYear,
    NOT_REQUIRED, underlying_maturity_date: UnderlyingMaturityDate,
    NOT_REQUIRED, underlying_maturity_time: UnderlyingMaturityTime,
    NOT_REQUIRED, underlying_coupon_payment_date: UnderlyingCouponPaymentDate,
    NOT_REQUIRED, underlying_restructuring_type: UnderlyingRestructuringType,
    NOT_REQUIRED, underlying_seniority: UnderlyingSeniority,
    NOT_REQUIRED, underlying_notional_percentage_outstanding: UnderlyingNotionalPercentageOutstanding,
    NOT_REQUIRED, underlying_original_notional_percentage_outstanding: UnderlyingOriginalNotionalPercentageOutstanding,
    NOT_REQUIRED, underlying_attachment_point: UnderlyingAttachmentPoint,
    NOT_REQUIRED, underlying_detachment_point: UnderlyingDetachmentPoint,
    NOT_REQUIRED, underlying_issue_date: UnderlyingIssueDate,
    NOT_REQUIRED, underlying_repo_collateral_security_type: UnderlyingRepoCollateralSecurityType,
    NOT_REQUIRED, underlying_repurchase_term: UnderlyingRepurchaseTerm,
    NOT_REQUIRED, underlying_repurchase_rate: UnderlyingRepurchaseRate,
    NOT_REQUIRED, underlying_factor: UnderlyingFactor,
    NOT_REQUIRED, underlying_credit_rating: UnderlyingCreditRating,
    NOT_REQUIRED, underlying_instr_registry: UnderlyingInstrRegistry,
    NOT_REQUIRED, underlying_country_of_issue: UnderlyingCountryOfIssue,
    NOT_REQUIRED, underlying_state_or_province_of_issue: UnderlyingStateOrProvinceOfIssue,
    NOT_REQUIRED, underlying_locale_of_issue: UnderlyingLocaleOfIssue,
    NOT_REQUIRED, underlying_redemption_date: UnderlyingRedemptionDate,
    NOT_REQUIRED, underlying_strike_price: UnderlyingStrikePrice,
    NOT_REQUIRED, underlying_strike_currency: UnderlyingStrikeCurrency,
    NOT_REQUIRED, underlying_opt_attribute: UnderlyingOptAttribute,
    NOT_REQUIRED, underlying_contract_multiplier: UnderlyingContractMultiplier,
    NOT_REQUIRED, underlying_contract_multiplier_unit: UnderlyingContractMultiplierUnit,
    NOT_REQUIRED, underlying_flow_schedule_type: UnderlyingFlowScheduleType,
    NOT_REQUIRED, underlying_unit_of_measure: UnderlyingUnitOfMeasure,
    NOT_REQUIRED, underlying_unit_of_measure_qty: UnderlyingUnitOfMeasureQty,
    NOT_REQUIRED, underlying_price_unit_of_measure: UnderlyingPriceUnitOfMeasure,
    NOT_REQUIRED, underlying_price_unit_of_measure_qty: UnderlyingPriceUnitOfMeasureQty,
    NOT_REQUIRED, underlying_time_unit: UnderlyingTimeUnit,
    NOT_REQUIRED, underlying_exercise_style: UnderlyingExerciseStyle,
    NOT_REQUIRED, underlying_coupon_rate: UnderlyingCouponRate,
    NOT_REQUIRED, underlying_security_exchange: UnderlyingSecurityExchange,
    NOT_REQUIRED, underlying_issuer: UnderlyingIssuer,
    NOT_REQUIRED, encoded_underlying_issuer_len: EncodedUnderlyingIssuerLen,
    NOT_REQUIRED, encoded_underlying_issuer: EncodedUnderlyingIssuer,
    NOT_REQUIRED, underlying_security_desc: UnderlyingSecurityDesc,
    NOT_REQUIRED, encoded_underlying_security_desc_len: EncodedUnderlyingSecurityDescLen,
    NOT_REQUIRED, encoded_underlying_security_desc: EncodedUnderlyingSecurityDesc,
    NOT_REQUIRED, underlying_cp_program: UnderlyingCPProgram,
    NOT_REQUIRED, underlying_cp_reg_type: UnderlyingCPRegType,
    NOT_REQUIRED, underlying_allocation_percent: UnderlyingAllocationPercent,
    NOT_REQUIRED, underlying_currency: UnderlyingCurrency,
    NOT_REQUIRED, underlying_qty: UnderlyingQty,
    NOT_REQUIRED, underlying_settlement_type: UnderlyingSettlementType,
    NOT_REQUIRED, underlying_cash_amount: UnderlyingCashAmount,
    NOT_REQUIRED, underlying_cash_type: UnderlyingCashType,
    NOT_REQUIRED, underlying_px: UnderlyingPx,
    NOT_REQUIRED, underlying_dirty_price: UnderlyingDirtyPrice,
    NOT_REQUIRED, underlying_end_price: UnderlyingEndPrice,
    NOT_REQUIRED, underlying_start_value: UnderlyingStartValue,
    NOT_REQUIRED, underlying_current_value: UnderlyingCurrentValue,
    NOT_REQUIRED, underlying_end_value: UnderlyingEndValue,
    NOT_REQUIRED, no_underlying_stips: NoUnderlyingStips,
    NOT_REQUIRED, underlying_adjusted_quantity: UnderlyingAdjustedQuantity,
    NOT_REQUIRED, underlying_fx_rate: UnderlyingFXRate,
    NOT_REQUIRED, underlying_fx_rate_calc: UnderlyingFXRateCalc,
    NOT_REQUIRED, underlying_cap_value: UnderlyingCapValue,
    NOT_REQUIRED, no_undly_instrument_parties: NoUndlyInstrumentParties,
    NOT_REQUIRED, underlying_settl_method: UnderlyingSettlMethod,
    NOT_REQUIRED, underlying_put_or_call: UnderlyingPutOrCall,
});

define_message!(UndlyInstrumentPtysSubGrp {
    REQUIRED, underlying_instrument_party_sub_id: UnderlyingInstrumentPartySubID,
    REQUIRED, underlying_instrument_party_sub_id_type: UnderlyingInstrumentPartySubIDType,
});

define_message!(UnderlyingStipulation {
    REQUIRED, underlying_stip_type: UnderlyingStipType,
    REQUIRED, underlying_stip_value: UnderlyingStipValue,
});

define_message!(UndSecAltIDGrp {
    REQUIRED, underlying_security_alt_id: UnderlyingSecurityAltID,
    REQUIRED, underlying_security_alt_id_source: UnderlyingSecurityAltIDSource,
});

